/// Trait to convert a type into an HTTP response.
pub trait IntoResponse {
    /// Converts the type into an HTTP response.
    fn into_response(self) -> tauri::http::Response<Vec<u8>>;
}

/// A [Response](tauri::http::Response) response for directly returning a response in commands.
pub struct Response(pub tauri::http::Response<Vec<u8>>);

impl IntoResponse for Response {
    fn into_response(self) -> tauri::http::Response<Vec<u8>> {
        self.0
    }
}

impl<T: serde::Serialize> IntoResponse for T {
    fn into_response(self) -> tauri::http::Response<Vec<u8>> {
        match serde_json::to_vec(&self) {
            Ok(body) => tauri::http::Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
            Err(e) => {
                let error = format!("Failed to serialize response: {}", e);
                let error = crate::Error::SerializationError(error);
                crate::response::error(error)
            }
        }
    }
}

/// Creates an error HTTP response from a crate::Error.
pub(crate) fn error(error: crate::Error) -> tauri::http::Response<Vec<u8>> {
    tauri::http::Response::builder()
        .status(error.status_code())
        .header("Content-Type", "application/json")
        .body(serde_json::to_vec(&error).unwrap_or_default())
        .unwrap()
}
