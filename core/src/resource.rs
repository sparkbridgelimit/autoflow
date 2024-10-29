use std::{cell::RefCell, rc::Rc};

use actix_service::{
    boxed::{self},
    fn_service, Service, ServiceFactory,
};
use futures_core::future::LocalBoxFuture;
use futures_util::future::join_all;

use crate::{
    config::AppService, error::Error, extractor::FromContext, handler::Handler, responder::Responder, response::TaskResponse, route::{Route, RouteService}, router::ResourceDef, service::{BoxedTaskService, BoxedTaskServiceFactory, HttpServiceFactory, ServiceRequest, ServiceResponse}
};

pub struct ResourceEndpoint {
    factory: Rc<RefCell<Option<ResourceFactory>>>,
}

impl ResourceEndpoint {
    fn new(factory: Rc<RefCell<Option<ResourceFactory>>>) -> Self {
        ResourceEndpoint { factory }
    }
}

impl ServiceFactory<ServiceRequest> for ResourceEndpoint {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = ResourceService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        self.factory.borrow().as_ref().unwrap().new_service(())
    }
}

pub struct Resource<T = ResourceEndpoint> {
    endpoint: T,
    // rdef: Patterns,
    name: Option<String>,
    routes: Vec<Route>,
    default: BoxedTaskServiceFactory,
    factory_ref: Rc<RefCell<Option<ResourceFactory>>>,
}

impl Resource {
    pub fn new(path: &str) -> Resource {
        let factory_ref = Rc::new(RefCell::new(None));

        Resource {
            routes: Vec::new(),
            name: Some(path.to_string()),
            endpoint: ResourceEndpoint::new(Rc::clone(&factory_ref)),
            factory_ref,
            default: boxed::factory(fn_service(|req: ServiceRequest| async {
                Ok(req.into_response(TaskResponse::new()))
            })),
        }
    }
}

impl Resource {
    pub fn to<F, Args>(mut self, handler: F) -> Self
    where
        F: Handler<Args>,
        Args: FromContext + 'static,
        F::Output: Responder + 'static,
    {
        self.routes.push(Route::new().to(handler));
        self
    }
}

impl<T> HttpServiceFactory for Resource<T>
where
    T: ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        > + 'static,
{
    fn register(self, config: &mut AppService) {
        let mut rdef = ResourceDef::new();

        if let Some(ref name) = self.name {
            rdef.set_name(name);
        }

        *self.factory_ref.borrow_mut() = Some(ResourceFactory {
            routes: self.routes,
            default: self.default,
        });

        config.register_service(rdef, self.endpoint)
    }
}

pub struct ResourceFactory {
    routes: Vec<Route>,
    default: BoxedTaskServiceFactory,
}

impl ServiceFactory<ServiceRequest> for ResourceFactory {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = ResourceService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        let default_fut = self.default.new_service(());

        let factory_fut = join_all(self.routes.iter().map(|route| route.new_service(())));

        Box::pin(async move {
            let default = default_fut.await?;
            let routes = factory_fut
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ResourceService { routes, default })
        })
    }
}

pub struct ResourceService {
    routes: Vec<RouteService>,
    default: BoxedTaskService,
}

impl Service<ServiceRequest> for ResourceService {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::always_ready!();

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        for route in &self.routes {
            if route.check(&mut req) {
                return route.call(req);
            }
        }

        self.default.call(req)
    }
}
