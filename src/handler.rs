use tauri::Runtime;

use crate::extract::FromRequest;
use crate::extract::FromRequestParts;
use crate::IntoResponse;

use super::CommandContext;

/// Trait to convert a function into a command handler.
pub trait CommandHandler<R: Runtime, T>: Clone + Send + Sync + Sized + 'static {
    fn call(
        self,
        req: tauri::http::Request<Vec<u8>>,
        ctx: CommandContext<R>,
    ) -> tauri::http::Response<Vec<u8>>;
}

/// Type-erased command handler function
pub(crate) type ErasedCommandHandler<R> = Box<
    dyn Fn(CommandContext<R>, tauri::http::Request<Vec<u8>>) -> tauri::http::Response<Vec<u8>>
        + Send
        + Sync,
>;

// Handler with no arguments
impl<F, R, Ret> CommandHandler<R, ()> for F
where
    F: FnOnce() -> Ret + Clone + Send + Sync + 'static,
    Ret: IntoResponse + 'static,
    R: tauri::Runtime,
{
    fn call(
        self,
        _req: tauri::http::Request<Vec<u8>>,
        _ctx: CommandContext<R>,
    ) -> tauri::http::Response<Vec<u8>> {
        self().into_response()
    }
}

macro_rules! impl_command_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<R, F, Res, M, $($ty,)* $last> CommandHandler<R, (M, $($ty,)* $last,)> for F
        where
            R: tauri::Runtime,
            F: FnOnce($($ty,)* $last) -> Res + Clone + Send + Sync + 'static,
            Res: IntoResponse,
            $($ty: FromRequestParts<R>,)*
            $last: FromRequest<R, M>,
        {
            fn call(
                self,
                req: tauri::http::Request<Vec<u8>>,
                mut ctx: CommandContext<R>,
            ) -> tauri::http::Response<Vec<u8>> {
                let (mut parts, body) = req.into_parts();

                $(
                    let $ty = match $ty::from_request_parts(&mut parts, &body, &mut ctx) {
                        Ok(value) => value,
                        Err(error) => return crate::response::error(error),
                    };
                )*

                let req = tauri::http::Request::from_parts(parts, body);

                let $last = match $last::from_request(req, &mut ctx) {
                    Ok(value) => value,
                    Err(error) => return crate::response::error(error),
                };

                self($($ty,)* $last).into_response()
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
