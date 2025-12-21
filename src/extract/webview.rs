use tauri::Runtime;

use crate::{CommandContext, FromRequestParts};

/// A [tauri::Webview] Extractor.
#[derive(Clone)]
pub struct Webview<R: Runtime>(tauri::Webview<R>);

impl<R: Runtime> FromRequestParts<R> for Webview<R> {
    async fn from_request_parts(
        _parts: &mut tauri::http::request::Parts,
        _body: &[u8],
        ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        ctx.webview().map(Self)
    }
}

impl<R: Runtime> std::ops::Deref for Webview<R> {
    type Target = tauri::Webview<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Runtime> std::ops::DerefMut for Webview<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
