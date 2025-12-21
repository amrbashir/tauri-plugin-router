use std::sync::Arc;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod context;
mod error;
mod extract;
mod handler;
mod response;
mod router;

pub use context::*;
pub use error::*;
pub use extract::*;
pub use handler::*;
pub use response::*;
pub use router::*;

/// Initializes the plugin.
///
/// ## Example
/// ```rust,no_run
/// use tauri::Runtime;
/// use tauri_plugin_router::{init, Router};
///
/// fn greet(name: String) -> String {
///    format!("Hello, {}!", name)
/// }
///
/// async fn greet_with_result(name: String) -> Result<String, String> {
///   // Simulate async work
///   tokio::time::sleep(std::time::Duration::from_secs(1)).await;
///     
///   if name.is_empty() {
///      Err("Name cannot be empty".to_string())
///   } else {
///     Ok(format!("Hello, {}!", name))
///   }
/// }
///
/// fn main() {
///     let router: Router<_> = Router::new()
///         .command("greet", greet)
///         .command("greet_with_result", greet_with_result);
///
///     let app = tauri::Builder::default()
///         .plugin(tauri_plugin_router::init(router));
/// }
/// ```
pub fn init<R: Runtime>(router: Router<R>) -> TauriPlugin<R> {
    let router = Arc::new(router);

    Builder::new("router")
        .register_asynchronous_uri_scheme_protocol("router", move |context, request, responder| {
            use tauri::http::header::*;
            use tauri::http::*;

            let app_handle = context.app_handle().clone();
            let webview_label = context.webview_label().to_string();
            let router = Arc::clone(&router);

            tauri::async_runtime::spawn(async move {
                let response = match *request.method() {
                    Method::OPTIONS => Response::builder()
                        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
                        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
                        .body(Vec::new())
                        .unwrap(),

                    Method::POST => {
                        let mut response = router
                            .handle_request(&app_handle, &webview_label, request)
                            .await;
                        let headers_mut = response.headers_mut();
                        headers_mut
                            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
                        headers_mut
                            .insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
                        response
                    }

                    _ => Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .header(CONTENT_TYPE, "application/json")
                        .body("only POST and OPTIONS are allowed".as_bytes().to_vec())
                        .unwrap(),
                };

                responder.respond(response);
            });
        })
        .build()
}
