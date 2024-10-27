use std::rc::Rc;

use actix_service::{boxed, IntoServiceFactory, ServiceFactory};

use crate::{
    error::Error,
    service::{BoxedHttpServiceFactory, ServiceRequest, ServiceResponse},
};

#[derive(Debug, Clone)]
pub struct AppConfig {}

pub struct AppService {
    config: AppConfig,
    root: bool,
    default: Rc<BoxedHttpServiceFactory>,
    #[allow(clippy::type_complexity)]
    services: Vec<BoxedHttpServiceFactory>,
}

impl AppService {
    pub fn register_service<F, S>(&mut self, factory: F)
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
        self.services.push(boxed::factory(factory.into_factory()));
    }
}
