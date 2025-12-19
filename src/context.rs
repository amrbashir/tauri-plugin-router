use tauri::{Manager, Runtime};

/// Context for a command execution.
///
/// This struct provides access to the application handle and webview label.
pub struct CommandContext<R: Runtime> {
    pub(crate) app_handle: tauri::AppHandle<R>,
    pub(crate) webview_label: String,
    pub(crate) json_args: Option<std::vec::IntoIter<serde_json::Value>>,
}

impl<R: Runtime> CommandContext<R> {
    /// Returns a reference to the application handle.
    pub fn app_handle(&self) -> &tauri::AppHandle<R> {
        &self.app_handle
    }

    /// Returns the label of the webview that made the request.
    pub fn webview_label(&self) -> &str {
        &self.webview_label
    }

    /// Returns the webview window that made the request.
    pub fn webview_window(&self) -> crate::Result<tauri::WebviewWindow<R>> {
        self.app_handle
            .get_webview_window(&self.webview_label)
            .ok_or_else(|| crate::Error::WebviewNotFound(self.webview_label.clone()))
    }

    /// Returns the webview that made the request.
    #[cfg(feature = "unstable")]
    pub fn webview(&self) -> crate::Result<tauri::Webview<R>> {
        self.app_handle
            .get_webview(&self.webview_label)
            .ok_or_else(|| crate::Error::WebviewNotFound(self.webview_label.clone()))
    }

    /// Returns the window that made the request.
    #[cfg(feature = "unstable")]
    pub fn window(&self) -> crate::Result<tauri::Window<R>> {
        self.webview().map(|w| w.window())
    }

    /// Takes the next JSON argument from the request body.
    pub(crate) fn take_json_arg(&mut self, body: &[u8]) -> crate::Result<serde_json::Value> {
        // If json_args is not initialized, parse the request body
        if self.json_args.is_none() {
            let json_args: Vec<serde_json::Value> = serde_json::from_slice(body).map_err(|e| {
                crate::Error::DeserializationError(format!("Failed to parse request body: {}", e))
            })?;

            self.json_args = Some(json_args.into_iter());
        }

        // Get the next argument from the iterator
        self.json_args
            .as_mut()
            .unwrap()
            .next()
            .ok_or_else(|| crate::Error::InvalidArgs("no more arguments available".to_string()))
    }
}
