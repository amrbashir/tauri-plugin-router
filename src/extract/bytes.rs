use tauri::Runtime;

use crate::{FromRequest, IntoResponse};

/// A bytes Extractor / Response.
pub struct Bytes(pub Vec<u8>);

impl<R: Runtime> FromRequest<R> for Bytes {
    fn from_request(
        req: tauri::http::Request<Vec<u8>>,
        _ctx: &mut crate::CommandContext<R>,
    ) -> crate::Result<Self> {
        Ok(Bytes(req.into_body()))
    }
}

impl IntoResponse for Bytes {
    fn into_response(self) -> tauri::http::Response<Vec<u8>> {
        tauri::http::Response::builder()
            .header("Content-Type", "application/octet-stream")
            .body(self.0)
            .unwrap()
    }
}

impl std::ops::Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
