use std::{cell::RefCell, rc::Rc};

use actix_service::{IntoServiceFactory, ServiceFactory};

use crate::{
    error::Error,
    service::{
        AppServiceFactory, BoxedTaskServiceFactory, HttpServiceFactory, ServiceFactoryWrapper,
        ServiceRequest, ServiceResponse,
    },
    task::Task,
    worker_service::{WorkerEntry, WorkerFactory, WorkerRoutingFactory},
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
        self.services
            .push(Box::new(ServiceFactoryWrapper::new(factory)));
        self
    }
}

impl<T> IntoServiceFactory<WorkerFactory<T>, Task> for Worker<T>
where
    T: ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        > + 'static,
{
    fn into_factory(self) -> WorkerFactory<T> {
        WorkerFactory {
            endpoint: self.endpoint,
            services: Rc::new(RefCell::new(self.services)),
            default: self.default,
            factory_ref: self.factory_ref,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use actix_service::{IntoServiceFactory, Service, ServiceFactory};

    use crate::{responder::Responder, task::Task, web};

    use super::Worker;

    pub async fn hello() -> impl Responder {
        Ok("world")
    }

    pub async fn some_task_1() -> impl Responder {
        Ok("some task 1")
    }

    pub async fn some_task_2() -> impl Responder {
        Ok("some task 2")
    }

    #[actix_rt::test]
    async fn test_default_resource() {
        let app = Worker::new().service(web::resource("hello_task").to(hello));
        let factory = app.into_factory();
        let srv = factory.new_service(()).await.unwrap();
        let task = Task::new("hello_task");
        let resp = srv.call(task).await.unwrap();
        println!("{:?}", resp.response());
    }

    #[actix_rt::test]
    async fn test_two_resource() {
        let r1 = web::resource("some_task_1").to(some_task_1);
        let r2 = web::resource("some_task_2").to(some_task_2);
        let app: Worker<crate::worker_service::WorkerEntry> = Worker::new().service(r1).service(r2);
        let factory = app.into_factory();
        let srv = factory.new_service(()).await.unwrap();
        let task = Task::new("some_task_2");
        let resp = srv.call(task).await.unwrap();
        println!("{:?}", resp.response());
    }

    #[actix_rt::test]
    async fn test_cost() {
        // 记录开始时间
        let start = Instant::now();

        let app = Worker::new().service(web::resource("hello").to(hello));
        let factory = app.into_factory();
        let srv = factory.new_service(()).await.unwrap();
        let task = Task::new("hello_task");
        let task = Task::new("");
        let resp = srv.call(task).await.unwrap();

        // 打印响应结果
        println!("{:?}", resp.response());

        // 记录结束时间并计算耗时
        let duration = start.elapsed();
        println!("请求耗时: {:?}", duration);
    }
}
