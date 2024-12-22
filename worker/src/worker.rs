use std::{cell::RefCell, rc::Rc};

use actix_service::IntoServiceFactory;

use crate::{
    service::{
        AppServiceFactory, BoxedTaskServiceFactory, HttpServiceFactory, ServiceFactoryWrapper,
    },
    task::Task,
    worker_service::WorkerFactory,
};

pub struct Worker {
    services: Vec<Box<dyn AppServiceFactory>>,
    default: Option<Rc<BoxedTaskServiceFactory>>,
}

impl Worker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {

        Worker {
            services: Vec::new(),
            default: None,
        }
    }
}

impl Worker{
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: HttpServiceFactory + 'static,
    {
        self.services
            .push(Box::new(ServiceFactoryWrapper::new(factory)));
        self
    }
}

impl IntoServiceFactory<WorkerFactory, Task> for Worker
{
    fn into_factory(self) -> WorkerFactory {
        WorkerFactory {
            services: Rc::new(RefCell::new(self.services)),
            default: self.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

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
