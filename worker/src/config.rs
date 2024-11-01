use std::rc::Rc;

use actix_service::{boxed, IntoServiceFactory, ServiceFactory};

use crate::{
    error::Error,
    router::ResourceDef,
    service::{BoxedTaskServiceFactory, ServiceRequest, ServiceResponse},
};

pub struct AppService {
    default: Rc<BoxedTaskServiceFactory>,
    #[allow(clippy::type_complexity)]
    services: Vec<(ResourceDef, BoxedTaskServiceFactory)>,
}

impl AppService {
    pub(crate) fn new(default: Rc<BoxedTaskServiceFactory>) -> Self {
        AppService {
            default,
            services: Vec::new(),
        }
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn into_services(self) -> Vec<(ResourceDef, BoxedTaskServiceFactory)> {
        self.services
    }

    pub fn register_service<F, S>(&mut self, rdef: ResourceDef, factory: F)
    where
        F: IntoServiceFactory<S, ServiceRequest>,
        S: ServiceFactory<
                ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                Config = (),
                InitError = (),
            > + 'static,
    {
        self.services
            .push((rdef, boxed::factory(factory.into_factory())));
    }
}
