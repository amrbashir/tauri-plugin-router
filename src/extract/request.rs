use tauri::Runtime;

use crate::{CommandContext, FromRequest};

/// A [Request] Extractor.
pub struct Request(pub tauri::http::Request<Vec<u8>>);

impl<R: Runtime> FromRequest<R> for Request {
    fn from_request(
        req: tauri::http::Request<Vec<u8>>,
        _ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        Ok(Request(req))
    }
}

impl std::ops::Deref for Request {
    type Target = tauri::http::Request<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
