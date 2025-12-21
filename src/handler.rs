use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tauri::Runtime;

use crate::extract::FromRequest;
use crate::extract::FromRequestParts;
use crate::IntoResponse;

use super::CommandContext;

mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum ViaSync {}
    #[derive(Debug, Clone, Copy)]
    pub enum ViaAsync {}
}

/// Trait to convert a function into a command handler.
pub trait CommandHandler<R: Runtime, T>: Clone + Send + Sync + Sized + 'static {
    type Future: Future<Output = tauri::http::Response<Vec<u8>>> + Send + 'static;

    fn call(self, req: tauri::http::Request<Vec<u8>>, ctx: CommandContext<R>) -> Self::Future;
}

/// Type-erased command handler function
pub(crate) type ErasedCommandHandler<R> = Arc<
    dyn Fn(
            CommandContext<R>,
            tauri::http::Request<Vec<u8>>,
        ) -> Pin<Box<dyn Future<Output = tauri::http::Response<Vec<u8>>> + Send>>
        + Send
        + Sync,
>;

// Handler with no arguments - sync version
impl<F, R, Ret> CommandHandler<R, (private::ViaSync,)> for F
where
    F: FnOnce() -> Ret + Clone + Send + Sync + 'static,
    Ret: IntoResponse + Send + 'static,
    R: tauri::Runtime,
{
    type Future = std::future::Ready<tauri::http::Response<Vec<u8>>>;

    fn call(self, _req: tauri::http::Request<Vec<u8>>, _ctx: CommandContext<R>) -> Self::Future {
        std::future::ready(self().into_response())
    }
}

// Handler with no arguments - async version
impl<F, R, Fut, Ret> CommandHandler<R, (private::ViaAsync,)> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Ret> + Send + 'static,
    Ret: IntoResponse + Send + 'static,
    R: tauri::Runtime,
{
    type Future = Pin<Box<dyn Future<Output = tauri::http::Response<Vec<u8>>> + Send>>;

    fn call(self, _req: tauri::http::Request<Vec<u8>>, _ctx: CommandContext<R>) -> Self::Future {
        Box::pin(async move { self().await.into_response() })
    }
}

macro_rules! impl_command_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        // Sync handler variant
        #[allow(non_snake_case, unused_mut)]
        impl<R, F, Res, M, $($ty,)* $last> CommandHandler<R, (private::ViaSync, M, $($ty,)* $last,)> for F
        where
            R: tauri::Runtime,
            F: FnOnce($($ty,)* $last) -> Res + Clone + Send + Sync + 'static,
            Res: IntoResponse + Send + 'static,
            $($ty: FromRequestParts<R> + Send,)*
            $last: FromRequest<R, M> + Send,
        {
            type Future = Pin<Box<dyn Future<Output = tauri::http::Response<Vec<u8>>> + Send>>;

            fn call(
                self,
                req: tauri::http::Request<Vec<u8>>,
                mut ctx: CommandContext<R>,
            ) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, &body, &mut ctx).await {
                            Ok(value) => value,
                            Err(error) => return crate::response::error(error),
                        };
                    )*

                    let req = tauri::http::Request::from_parts(parts, body);

                    let $last = match $last::from_request(req, &mut ctx).await {
                        Ok(value) => value,
                        Err(error) => return crate::response::error(error),
                    };

                    self($($ty,)* $last).into_response()
                })
            }
        }

        // Async handler variant
        #[allow(non_snake_case, unused_mut)]
        impl<R, F, Fut, Res, M, $($ty,)* $last> CommandHandler<R, (private::ViaAsync, M, $($ty,)* $last,)> for F
        where
            R: tauri::Runtime,
            F: FnOnce($($ty,)* $last) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = Res> + Send + 'static,
            Res: IntoResponse + Send + 'static,
            $($ty: FromRequestParts<R> + Send,)*
            $last: FromRequest<R, M> + Send,
        {
            type Future = Pin<Box<dyn Future<Output = tauri::http::Response<Vec<u8>>> + Send>>;

            fn call(
                self,
                req: tauri::http::Request<Vec<u8>>,
                mut ctx: CommandContext<R>,
            ) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, &body, &mut ctx).await {
                            Ok(value) => value,
                            Err(error) => return crate::response::error(error),
                        };
                    )*

                    let req = tauri::http::Request::from_parts(parts, body);

                    let $last = match $last::from_request(req, &mut ctx).await {
                        Ok(value) => value,
                        Err(error) => return crate::response::error(error),
                    };

                    self($($ty,)* $last).await.into_response()
                })
            }
        }
    };
}

#[rustfmt::skip]
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
    };
}

all_the_tuples!(impl_command_handler);
