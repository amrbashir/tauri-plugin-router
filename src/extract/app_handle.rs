use tauri::Runtime;

use crate::{CommandContext, FromRequestParts};

/// A [tauri::AppHandle] Extractor.
#[derive(Clone)]
pub struct AppHandle<R: Runtime>(pub tauri::AppHandle<R>);

impl<R: Runtime> FromRequestParts<R> for AppHandle<R> {
    fn from_request_parts(
        _parts: &mut tauri::http::request::Parts,
        _body: &[u8],
        ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        Ok(Self(ctx.app_handle().clone()))
    }
}

impl<R: Runtime> std::ops::Deref for AppHandle<R> {
    type Target = tauri::AppHandle<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Runtime> std::ops::DerefMut for AppHandle<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
