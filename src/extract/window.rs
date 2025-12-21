use tauri::Runtime;

use crate::{CommandContext, FromRequestParts};

/// A [tauri::Window] Extractor.
#[cfg(feature = "unstable")]
#[derive(Clone)]
pub struct Window<R: Runtime>(tauri::Window<R>);

#[cfg(feature = "unstable")]
impl<R: Runtime> FromRequestParts<R> for Window<R> {
    async fn from_request_parts(
        _parts: &mut tauri::http::request::Parts,
        _body: &[u8],
        ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        ctx.window().map(Self)
    }
}

#[cfg(feature = "unstable")]
impl<R: Runtime> std::ops::Deref for Window<R> {
    type Target = tauri::Window<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "unstable")]
impl<R: Runtime> std::ops::DerefMut for Window<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
