use std::{cell::RefCell, rc::Rc};

use actix_service::{IntoServiceFactory, ServiceFactory};

use crate::{
    error::Error,
    service::{
        AppServiceFactory, BoxedTaskServiceFactory, HttpServiceFactory, ServiceFactoryWrapper,
        ServiceRequest, ServiceResponse,
    },
    task::Task,
    worker_service::WorkerFactory,
};

pub struct Worker<T> {
    endpoint: T,
    services: Vec<Box<dyn AppServiceFactory>>,
    default: Option<Rc<BoxedTaskServiceFactory>>,
}

impl Worker<()> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Worker {
            endpoint: (),
            services: Vec::new(),
            default: None,
        }
    }
}

impl<T> Worker<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    /// 向 Worker 注册一个实现了 HttpServiceFactory 的“资源”
    /// （典型场景：web::resource("task_name").to(handler)）
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: HttpServiceFactory + 'static,
    {
        self.services.push(Box::new(ServiceFactoryWrapper::new(factory)));
        self
    }

    /// 如果需要设置一个 default service
    pub fn default_service(mut self, default: Rc<BoxedTaskServiceFactory>) -> Self {
        self.default = Some(default);
        self
    }
}

/// 将 Worker<T> 转换为 `WorkerFactory<T>`，它实现了 `ServiceFactory<Task>`
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
