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
    services: Vec<(ResourceDef, Rc<BoxedTaskServiceFactory>)>,
}

impl AppService {
    pub(crate) fn new(default: Rc<BoxedTaskServiceFactory>) -> Self {
        AppService {
            default,
            services: Vec::new(),
        }
    }

    pub fn into_services(
        self,
    ) -> (
        Rc<BoxedTaskServiceFactory>,
        Vec<(ResourceDef, Rc<BoxedTaskServiceFactory>)>,
    ) {
        (self.default, self.services)
    }

    pub fn register_service<F, S>(&mut self, rdef: ResourceDef, factory: F)
    where
        F: IntoServiceFactory<S, ServiceRequest>,
        S: ServiceFactory<ServiceRequest, Response = ServiceResponse, Error = Error, Config = (), InitError = ()> + 'static,
    {
        let bfactory = boxed::factory(factory.into_factory());
        // wrap it
        let rc_factory = Rc::new(bfactory);

        self.services.push((rdef, rc_factory));
    }
}
