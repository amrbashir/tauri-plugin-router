use serde::de::DeserializeOwned;
use std::future::Future;
use tauri::Runtime;

use crate::CommandContext;

mod app_handle;
mod bytes;
mod header_map;
mod request;
#[cfg(feature = "unstable")]
mod webview;
mod webview_window;
#[cfg(feature = "unstable")]
mod window;

pub use app_handle::*;
pub use bytes::*;
pub use header_map::*;
pub use request::*;
#[cfg(feature = "unstable")]
pub use webview::*;
pub use webview_window::*;
#[cfg(feature = "unstable")]
pub use window::*;

mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum ViaParts {}

    #[derive(Debug, Clone, Copy)]
    pub enum ViaRequest {}
}

/// Trait for types that can be extracted from the request parts (headers, URI, method).
///
/// Extractors that implement this trait can only read from the request parts,
/// not the body. All extractors except the last one must implement this trait.
pub trait FromRequestParts<R: Runtime>: Sized {
    /// Extract this type from the request parts.
    fn from_request_parts(
        parts: &mut tauri::http::request::Parts,
        body: &[u8],
        ctx: &mut CommandContext<R>,
    ) -> impl Future<Output = crate::Result<Self>> + Send;
}

/// Trait for types that can be extracted from the complete request (including body).
///
/// Only the last extractor in a handler function can implement this trait,
/// as it consumes the request body.
pub trait FromRequest<R: Runtime, M = private::ViaRequest>: Sized {
    /// Extract this type from the request.
    fn from_request(
        req: tauri::http::Request<Vec<u8>>,
        ctx: &mut CommandContext<R>,
    ) -> impl Future<Output = crate::Result<Self>> + Send;
}

/// Blanket implementation to allow any `FromRequestParts` extractor to be used
/// as a `FromRequest` extractor.
impl<T: FromRequestParts<R>, R: Runtime> FromRequest<R, private::ViaParts> for T {
    async fn from_request(
        req: tauri::http::Request<Vec<u8>>,
        ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        let (mut parts, body) = req.into_parts();
        T::from_request_parts(&mut parts, &body, ctx).await
    }
}

/// Blanket implementation to allow deserializing JSON body into any type
/// that implements `DeserializeOwned`.
impl<R: Runtime, T: DeserializeOwned + Send + 'static> FromRequestParts<R> for T {
    async fn from_request_parts(
        _parts: &mut tauri::http::request::Parts,
        body: &[u8],
        ctx: &mut CommandContext<R>,
    ) -> crate::Result<Self> {
        let arg = ctx.take_json_arg(body)?;
        let deserialized = serde_json::from_value(arg).map_err(|e| {
            crate::Error::DeserializationError(format!("JSON deserialization error: {}", e))
        })?;
        Ok(deserialized)
    }
}
