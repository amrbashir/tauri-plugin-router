use tauri::Runtime;

use crate::{CommandContext, FromRequestParts};

/// A [tauri::WebviewWindow] Extractor.
#[derive(Clone)]
pub struct WebviewWindow<R: Runtime>(tauri::WebviewWindow<R>);

impl<R: Runtime> FromRequestParts<R> for WebviewWindow<R> {
    async fn from_request_parts(
        _parts: &mut tauri::http::request::Parts,
        _body: &[u8],
        ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        ctx.webview_window().map(Self)
    }
}

impl<R: Runtime> std::ops::Deref for WebviewWindow<R> {
    type Target = tauri::WebviewWindow<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Runtime> std::ops::DerefMut for WebviewWindow<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
