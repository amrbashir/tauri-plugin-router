use tauri::Runtime;

use crate::{CommandContext, FromRequestParts};

/// A [tauri::http::HeaderMap] Extractor.
pub struct HeaderMap(pub tauri::http::HeaderMap);

impl<R: Runtime> FromRequestParts<R> for HeaderMap {
    async fn from_request_parts(
        parts: &mut tauri::http::request::Parts,
        _body: &[u8],
        _ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        Ok(HeaderMap(parts.headers.clone()))
    }
}
