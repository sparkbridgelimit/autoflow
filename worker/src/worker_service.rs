use std::{cell::RefCell, mem, rc::Rc};

use actix_service::{boxed, fn_service, Service, ServiceFactory};
use futures_core::future::LocalBoxFuture;
use futures_util::future::join_all;

use crate::{
    config::AppService,
    context::TaskContext,
    error::Error,
    response::TaskResponse,
    router::{ResourceDef, TaskRouter},
    service::{
        AppServiceFactory, BoxedTaskService, BoxedTaskServiceFactory, ServiceRequest,
        ServiceResponse,
    },
    task::Task,
};

pub struct WorkerFactory {
    pub(crate) services: Rc<RefCell<Vec<Box<dyn AppServiceFactory>>>>,
    pub(crate) default: Option<Rc<BoxedTaskServiceFactory>>,
}

impl ServiceFactory<Task> for WorkerFactory {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = WorkerService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        let default = self.default.clone().unwrap_or_else(|| {
            Rc::new(boxed::factory(fn_service(|req: ServiceRequest| async {
                Ok(ServiceResponse::new(
                    req.into_parts(),
                    TaskResponse::success(serde_json::Value::Null),
                ))
            })))
        });

        // 复制一个default service的引用过去
        let mut config = AppService::new(Rc::clone(&default));

        // 注册到AppService
        mem::take(&mut *self.services.borrow_mut())
            .into_iter()
            .for_each(|mut srv| srv.register(&mut config));

        let services = config.into_services();

        let routes = config.into_services();

        Box::pin(async move { Ok(WorkerService { routes, default }) })
    }
}

pub struct WorkerService {
    router: TaskRouter<BoxedTaskService>, // 路由表
    default: BoxedTaskService,
}

impl Service<Task> for WorkerService {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // 因为路由表是静态的，始终可以处理请求
        std::task::Poll::Ready(Ok(()))
    }

    // 将原始的req转成ServiceReq
    fn call(&self, req: Task) -> Self::Future {
        // 创建 ServiceRequest
        let ctx = TaskContext::new(req.name.as_str());
        let sreq = ServiceRequest::new(ctx);

        todo!()
    }
}

pub struct WorkerRouting {
    router: TaskRouter<BoxedTaskService>,
    default: BoxedTaskService,
}

impl Service<ServiceRequest> for WorkerRouting {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::always_ready!();

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(srv) = self.router.recognize(&req) {
            srv.call(req)
        } else {
            self.default.call(req)
        }
    }
}

pub struct WorkerRoutingFactory {
    #[allow(clippy::type_complexity)]
    services: Rc<[(ResourceDef, BoxedTaskServiceFactory)]>,
    default: Rc<BoxedTaskServiceFactory>,
}

impl ServiceFactory<ServiceRequest> for WorkerRoutingFactory {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = WorkerRouting;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        let factory_fut = join_all(self.services.iter().map(|(path, factory)| {
            let path = path.clone();
            let factory_fut = factory.new_service(());
            async move { factory_fut.await.map(move |service| (path, service)) }
        }));

        // construct default service factory future
        let default_fut = self.default.new_service(());

        Box::pin(async move {
            let default = default_fut.await?;

            // build router from the factory future result.
            let router = factory_fut
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .drain(..)
                .fold(TaskRouter::build(), |mut router, (path, service)| {
                    router.push(path, service);
                    router
                })
                .finish();

            Ok(WorkerRouting { router, default })
        })
    }
}

pub struct WorkerEntry {
    factory: Rc<RefCell<Option<WorkerRoutingFactory>>>,
}

impl WorkerEntry {
    pub fn new(factory: Rc<RefCell<Option<WorkerRoutingFactory>>>) -> Self {
        WorkerEntry { factory }
    }
}

impl ServiceFactory<ServiceRequest> for WorkerEntry {
    type Response = ServiceResponse;
    type Error = Error;
    type Config = ();
    type Service = WorkerRouting;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        self.factory.borrow_mut().as_mut().unwrap().new_service(())
    }
}
