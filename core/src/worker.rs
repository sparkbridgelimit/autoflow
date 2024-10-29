use std::{cell::RefCell, rc::Rc};

use actix_service::{IntoServiceFactory, ServiceFactory};

use crate::{
    error::Error, service::{AppServiceFactory, BoxedTaskServiceFactory, HttpServiceFactory, ServiceFactoryWrapper, ServiceRequest, ServiceResponse}, task::Task, worker_service::{WorkerEntry, WorkerInit, WorkerRoutingFactory}
};

pub struct Worker<T> {
    endpoint: T,
    services: Vec<Box<dyn AppServiceFactory>>,
    default: Option<Rc<BoxedTaskServiceFactory>>,
    factory_ref: Rc<RefCell<Option<WorkerRoutingFactory>>>,
}

impl Worker<WorkerEntry> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let factory_ref = Rc::new(RefCell::new(None));

        Worker {
            endpoint: WorkerEntry::new(Rc::clone(&factory_ref)),
            services: Vec::new(),
            default: None,
            factory_ref,
        }
    }
}

impl<T> Worker<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: HttpServiceFactory + 'static,
    {
        self.services.push(Box::new(ServiceFactoryWrapper::new(factory)));
        self
    }
}

impl<T> IntoServiceFactory<WorkerInit<T>, Task> for Worker<T>
where
    T: ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        > + 'static,
{
    fn into_factory(self) -> WorkerInit<T> {
        WorkerInit {
            endpoint: self.endpoint,
            services: Rc::new(RefCell::new(self.services)),
            default: self.default,
            factory_ref: self.factory_ref,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn arg_number() {

    }

}
