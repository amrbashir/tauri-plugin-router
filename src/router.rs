use std::collections::HashMap;

use tauri::Runtime;

use crate::{CommandContext, CommandHandler, ErasedCommandHandler};

/// The router that holds command handlers and dispatches requests.
pub struct Router<R: Runtime> {
    pub(crate) commands: HashMap<String, ErasedCommandHandler<R>>,
}

impl<R: Runtime> Default for Router<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Runtime> Router<R> {
    /// Creates a new empty router.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Registers a command handler for the given command name.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use tauri::Runtime;
    /// use tauri_plugin_router::{init, Router};
    ///
    /// fn greet(name: String) -> String {
    ///    format!("Hello, {}!", name)
    /// }
    /// fn main() {
    ///     let router: Router<_> = Router::new()
    ///         .command("greet", greet);
    ///
    ///     let app = tauri::Builder::default()
    ///         .plugin(tauri_plugin_router::init(router));
    /// }
    /// ```
    pub fn command<H, T>(mut self, cmd: &str, handler: H) -> Self
    where
        H: CommandHandler<R, T>,
    {
        let erased: ErasedCommandHandler<R> =
            std::sync::Arc::new(move |ctx, req| Box::pin(handler.clone().call(req, ctx)));
        self.commands.insert(cmd.to_string(), erased);
        self
    }

    /// Handles an incoming request by dispatching it to the appropriate command handler.
    pub(crate) async fn handle_request(
        &self,
        app_handle: &tauri::AppHandle<R>,
        webview_label: &str,
        request: tauri::http::Request<Vec<u8>>,
    ) -> tauri::http::Response<Vec<u8>> {
        // Extract command name from URI path
        let command_name = request.uri().path().trim_start_matches('/').to_string();

        // Create CommandContext from Tauri context with request ownership
        let ctx = CommandContext {
            app_handle: app_handle.clone(),
            webview_label: webview_label.to_string(),
            json_args: None,
        };

        // Find and execute the command handler
        match self.commands.get(&command_name) {
            Some(handler) => handler(ctx, request).await,
            None => {
                let error = crate::Error::CommandNotFound(command_name);
                crate::response::error(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Test utilities
    // --------------

    macro_rules! call_raw {
        ($router:expr, $app:expr, $command:expr, $body:expr) => {{
            $router
                .handle_request(
                    $app.handle(),
                    "test_webview",
                    tauri::http::Request::builder()
                        .uri(format!("router://localhost/{}", $command))
                        .body($body)
                        .unwrap(),
                )
                .await
        }};
    }

    macro_rules! call_json {
        ($router:expr, $app:expr, $command:expr, $args:expr) => {{
            let body = serde_json::to_vec($args).unwrap();
            call_raw!($router, $app, $command, body)
        }};
    }

    macro_rules! body_as_string {
        ($response:expr) => {
            String::from_utf8($response.into_body()).unwrap()
        };
    }

    macro_rules! body_as_json {
        ($response:expr) => {
            serde_json::from_slice(&$response.into_body()).unwrap()
        };
    }

    // Sample command handlers for testing
    // -----------------------------------

    fn greet(name: String) -> String {
        format!("Hello, {}!", name)
    }

    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    fn calc(a: f64, b: f64, operation: Operation) -> std::result::Result<f64, String> {
        match operation {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }

    fn no_args() -> String {
        "Hello from no args!".to_string()
    }

    fn unit_return(_x: u32) {}

    fn with_app<R: Runtime>(_app: AppHandle<R>, name: String) -> String {
        format!("With App, {}!", name)
    }

    #[derive(serde::Deserialize, serde::Serialize, Clone)]
    struct Message {
        content: String,
    }

    fn raw_to_json(Bytes(bytes): Bytes) -> Message {
        Message {
            content: format!("Raw to json, received {} bytes", bytes.len()),
        }
    }

    fn json_to_raw(data: Message) -> Bytes {
        Bytes(serde_json::to_vec(&data).unwrap())
    }

    fn request_response(Request(req): Request) -> Response {
        Response(
            tauri::http::Response::builder()
                .header("Content-Type", "application/octet-stream")
                .body(req.into_body())
                .unwrap(),
        )
    }

    // Async handler examples for testing
    async fn async_greet(name: String) -> String {
        format!("Hello async, {}!", name)
    }

    async fn async_add(a: u32, b: u32) -> u32 {
        // Simulate some async work (without external dependencies)
        a + b
    }

    async fn async_with_result(value: i32) -> std::result::Result<i32, String> {
        if value < 0 {
            Err("Negative value not allowed".to_string())
        } else {
            Ok(value * 2)
        }
    }

    async fn async_with_app<R: Runtime>(_app: AppHandle<R>, name: String) -> String {
        format!("Async with App, {}!", name)
    }

    #[tokio::test]
    async fn it_works() {
        let app = tauri::test::mock_app();

        let router = Router::new()
            .command("greet", greet)
            .command("add", add)
            .command("calc", calc)
            .command("no_args", no_args)
            .command("unit_return", unit_return)
            .command("with_app", with_app)
            .command("raw_to_json", raw_to_json)
            .command("json_to_raw", json_to_raw)
            .command("request_response", request_response)
            .command("async_greet", async_greet)
            .command("async_add", async_add)
            .command("async_with_result", async_with_result)
            .command("async_with_app", async_with_app);

        // test sync handlers
        let response = call_json!(router, app, "greet", &["Tauri"]);
        assert_eq!(body_as_string!(response), "\"Hello, Tauri!\"");

        let response = call_json!(router, app, "add", &[3, 5]);
        assert_eq!(body_as_string!(response), "8");

        let response = call_json!(router, app, "calc", &(10.0, 2.0, Operation::Multiply));
        assert_eq!(body_as_string!(response), "{\"Ok\":20.0}"); // TODO: fix Result serialization

        let response = call_json!(router, app, "no_args", &());
        assert_eq!(body_as_string!(response), "\"Hello from no args!\"");

        let response = call_json!(router, app, "unit_return", &[42]);
        assert_eq!(body_as_string!(response), "null");

        let response = call_json!(router, app, "with_app", &["Tauri"]);
        assert_eq!(body_as_string!(response), "\"With App, Tauri!\"");

        let response = call_raw!(router, app, "raw_to_json", b"hello world".to_vec());
        let msg: Message = body_as_json!(response);
        assert_eq!(msg.content, "Raw to json, received 11 bytes");

        let message = Message {
            content: "Hello, JSON!".to_string(),
        };
        let message_clone = message.clone();
        let response = call_json!(router, app, "json_to_raw", &[message_clone]);
        assert_eq!(response.into_body(), serde_json::to_vec(&message).unwrap());

        let response = call_raw!(router, app, "request_response", b"echo this".to_vec());
        assert_eq!(response.into_body(), b"echo this".to_vec());

        // test async handlers
        let response = call_json!(router, app, "async_greet", &["World"]);
        assert_eq!(body_as_string!(response), "\"Hello async, World!\"");

        let response = call_json!(router, app, "async_add", &[7, 3]);
        assert_eq!(body_as_string!(response), "10");

        let response = call_json!(router, app, "async_with_result", &[5]);
        assert_eq!(body_as_string!(response), "{\"Ok\":10}");

        let response = call_json!(router, app, "async_with_result", &[-5]);
        assert_eq!(
            body_as_string!(response),
            "{\"Err\":\"Negative value not allowed\"}"
        );

        let response = call_json!(router, app, "async_with_app", &["Async"]);
        assert_eq!(body_as_string!(response), "\"Async with App, Async!\"");
    }
}
