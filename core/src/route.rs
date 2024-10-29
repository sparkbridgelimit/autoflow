use actix_service::{boxed::{self, BoxService}, fn_service, Service, ServiceFactory};
use futures_core::future::LocalBoxFuture;

use crate::{
    error::Error, extractor::FromContext, handler::{handler_service, Handler}, responder::Responder, response::TaskResponse, service::{BoxedTaskServiceFactory, ServiceRequest, ServiceResponse}
};

pub struct Route {
    service: BoxedTaskServiceFactory,
}

impl Route {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Route {
        Route {
            service: boxed::factory(fn_service(|req: ServiceRequest| async {
                Ok(req.into_response(TaskResponse::new()))
            })),
        }
    }
}

impl ServiceFactory<ServiceRequest> for Route {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = RouteService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        let fut = self.service.new_service(());

        Box::pin(async move {
            let service = fut.await?;
            Ok(RouteService { service })
        })
    }
}

impl Route {
    pub fn to<F, Args>(mut self, handler: F) -> Self
    where
        F: Handler<Args>,
        Args: FromContext + 'static,
        F::Output: Responder + 'static,
    {
        self.service = handler_service(handler);
        self
    }
}

pub struct RouteService {
    service: BoxService<ServiceRequest, ServiceResponse, Error>,
}

impl RouteService {
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn check(&self, req: &mut ServiceRequest) -> bool {
        true
    }
}

impl Service<ServiceRequest> for RouteService {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        self.service.call(req)
    }
}