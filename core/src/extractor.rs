use crate::{context::TaskContext, error::Error};
use futures_core::ready;
use pin_project_lite::pin_project;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use actix_utils::future::{ok, Ready};

pub trait FromContext: Sized {
    type Error: Into<Error>;

    type Future: Future<Output = Result<Self, Self::Error>>;

    fn from_context(req: &TaskContext) -> Self::Future;
}

impl<T> FromContext for Option<T>
where
    T: FromContext,
{
    type Error = Infallible;
    type Future = FromContextOptFuture<T::Future>;

    #[inline]
    fn from_context(ctx: &TaskContext) -> Self::Future {
        FromContextOptFuture {
            fut: T::from_context(ctx),
        }
    }
}

pin_project! {
    pub struct FromContextOptFuture<Fut> {
        #[pin]
        fut: Fut,
    }
}

impl<Fut, T, E> Future for FromContextOptFuture<Fut>
where
    Fut: Future<Output = Result<T, E>>,
    E: Into<Error>,
{
    type Output = Result<Option<T>, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = ready!(this.fut.poll(cx));
        match res {
            Ok(t) => Poll::Ready(Ok(Some(t))),
            Err(err) => {
                log::debug!("Error for Option<T> extractor: {}", err.into());
                Poll::Ready(Ok(None))
            }
        }
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
mod tuple_from_ctx {
    use super::*;

    macro_rules! tuple_from_ctx {
        ($fut: ident; $($T: ident),*) => {
            #[allow(unused_parens)]
            impl<$($T: FromContext + 'static),+> FromContext for ($($T,)+)
            {
                type Error = Error;
                type Future = $fut<$($T),+>;

                fn from_context(ctx: &TaskContext) -> Self::Future {
                    $fut {
                        $(
                            $T: ExtractFuture::Future {
                                fut: $T::from_context(ctx)
                            },
                        )+
                    }
                }
            }

            pin_project! {
                pub struct $fut<$($T: FromContext),+> {
                    $(
                        #[pin]
                        $T: ExtractFuture<$T::Future, $T>,
                    )+
                }
            }

            impl<$($T: FromContext),+> Future for $fut<$($T),+>
            {
                type Output = Result<($($T,)+), Error>;

                fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                    let mut this = self.project();

                    let mut ready = true;
                    $(
                        match this.$T.as_mut().project() {
                            ExtractProj::Future { fut } => match fut.poll(cx) {
                                Poll::Ready(Ok(output)) => {
                                    let _ = this.$T.as_mut().project_replace(ExtractFuture::Done { output });
                                },
                                Poll::Ready(Err(err)) => return Poll::Ready(Err(err.into())),
                                Poll::Pending => ready = false,
                            },
                            ExtractProj::Done { .. } => {},
                            ExtractProj::Empty => unreachable!("FromContext polled after finished"),
                        }
                    )+

                    if ready {
                        Poll::Ready(Ok(
                            ($(
                                match this.$T.project_replace(ExtractFuture::Empty) {
                                    ExtractReplaceProj::Done { output } => output,
                                    _ => unreachable!("FromContext polled after finished"),
                                },
                            )+)
                        ))
                    } else {
                        Poll::Pending
                    }
                }
            }
        };
    }

    pin_project! {
        #[project = ExtractProj]
        #[project_replace = ExtractReplaceProj]
        enum ExtractFuture<Fut, Res> {
            Future {
                #[pin]
                fut: Fut
            },
            Done {
                output: Res,
            },
            Empty
        }
    }

    impl FromContext for () {
        type Error = Infallible;
        type Future = Ready<Result<Self, Self::Error>>;

        fn from_context(_: &TaskContext) -> Self::Future {
            ok(())
        }
    }

    tuple_from_ctx! { TupleFromContext1; A }
    tuple_from_ctx! { TupleFromContext2; A, B }
    tuple_from_ctx! { TupleFromContext3; A, B, C }
    tuple_from_ctx! { TupleFromContext4; A, B, C, D }
    tuple_from_ctx! { TupleFromContext5; A, B, C, D, E }
    tuple_from_ctx! { TupleFromContext6; A, B, C, D, E, F }
    tuple_from_ctx! { TupleFromContext7; A, B, C, D, E, F, G }
    tuple_from_ctx! { TupleFromContext8; A, B, C, D, E, F, G, H }
    tuple_from_ctx! { TupleFromContext9; A, B, C, D, E, F, G, H, I }
    tuple_from_ctx! { TupleFromContext10; A, B, C, D, E, F, G, H, I, J }
    tuple_from_ctx! { TupleFromContext11; A, B, C, D, E, F, G, H, I, J, K }
    tuple_from_ctx! { TupleFromContext12; A, B, C, D, E, F, G, H, I, J, K, L }
    tuple_from_ctx! { TupleFromContext13; A, B, C, D, E, F, G, H, I, J, K, L, M }
    tuple_from_ctx! { TupleFromContext14; A, B, C, D, E, F, G, H, I, J, K, L, M, N }
    tuple_from_ctx! { TupleFromContext15; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O }
    tuple_from_ctx! { TupleFromContext16; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P }
}
