use std::{cell::RefCell, mem, rc::Rc};

use actix_service::{boxed, fn_service, Service, ServiceFactory};
use futures_core::future::LocalBoxFuture;

use crate::{
    config::AppService,
    context::TaskContext,
    error::Error,
    response::TaskResponse,
    service::{AppServiceFactory, BoxedTaskServiceFactory, ServiceRequest, ServiceResponse},
    task::Task,
    task_registry::TaskRegistry,
};

/// WorkerFactory 负责将用户注册的服务收集起来，最终在 new_service() 时构造 WorkerService。
pub struct WorkerFactory<T>
where
    T: ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
{
    /// 中间件、endpoint 等，可以在请求进入时执行额外逻辑
    pub(crate) endpoint: T,
    /// 用户通过 Worker::service(...) 注册的 “(ResourceDef, BoxedTaskServiceFactory)” 等集合
    pub(crate) services: Rc<RefCell<Vec<Box<dyn AppServiceFactory>>>>,
    /// 缺省服务（当未匹配到 task_name 时使用）
    pub(crate) default: Option<Rc<BoxedTaskServiceFactory>>,
}

/// 实现 Actix 的 ServiceFactory<Task>，让 WorkerFactory 可以处理 `Task`
impl<T> ServiceFactory<Task> for WorkerFactory<T>
where
    T: ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
    T::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = T::Error;
    type Config = ();
    type Service = WorkerService<T::Service>;
    type InitError = T::InitError;
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    /// 在 new_service() 中，将用户注册的 services 收集到一个 TaskRegistry，构造 WorkerService
    fn new_service(&self, _: ()) -> Self::Future {
        // 1. 如果用户没有设置 default，则创建一个默认的 factory，返回空响应
        let default = self.default.clone().unwrap_or_else(|| {
            Rc::new(boxed::factory(fn_service(|req: ServiceRequest| async {
                Ok(ServiceResponse::new(
                    req.into_parts(),
                    TaskResponse::success(serde_json::Value::Null),
                ))
            })))
        });

        // 2. 创建一个 AppService
        let mut config = AppService::new(Rc::clone(&default));

        // 3. 将 self.services 里存的 factory 全部注册到 config
        mem::take(&mut *self.services.borrow_mut())
            .into_iter()
            .for_each(|mut srv| srv.register(&mut config));

        // 4. 将 config 转换成 (default_factory, Vec<(ResourceDef, BoxedTaskServiceFactory)>)
        let (default_factory, items) = config.into_services();

        // 5. 创建一个 TaskRegistry，插入 (ResourceDef, factory)，保留默认工厂
        let mut registry = TaskRegistry::new(Rc::clone(&default_factory));
        for (rdef, fac) in items {
            registry.insert(rdef, fac); // 不需要 Rc::clone(&fac) 也可以, 取决于你是否想保留 origin
        }

        // 6. 构建 endpoint（你可以理解成全局中间件或 handler 的顶层服务）
        let endpoint_fut = self.endpoint.new_service(());

        // 7. 返回一个 future，异步等待 endpoint 构造完成，然后构造 WorkerService
        Box::pin(async move {
            let endpoint = endpoint_fut.await?;
            Ok(WorkerService {
                service: endpoint,
                registry: Rc::new(RefCell::new(registry)),
            })
        })
    }
}

/// WorkerService 在运行时实际处理 `Task`
/// 内部根据 task_name 查找对应的 service 并执行
pub struct WorkerService<T>
where
    T: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    /// endpoint: 可能是全局中间件 / 顶层 handler
    service: T,
    /// 将 “(ResourceDef, BoxedTaskServiceFactory)” 保存在这个 registry 里
    registry: Rc<RefCell<TaskRegistry>>,
}

impl<T> Service<Task> for WorkerService<T>
where
    T: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = T::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    // 将原始的req转成ServiceReq
    fn call(&self, task: Task) -> Self::Future {
        let name = task.name.clone();
        // 根据 task 构造一个 TaskContext
        let ctx = TaskContext::new(name.as_str());

        // 在 registry 中查找对应的 ServiceFactory
        let reg = self.registry.borrow();
        let factory = reg.get_factory(name.as_str()); // 如果没找到，会返回 default

        // 构造 ServiceRequest
        let req = ServiceRequest::new(ctx);

        // 调用 .new_service()
        let fut = factory.new_service(());

        // 异步执行
        Box::pin(async move {
            // 等待拿到真正的 Service
            let svc = fut.await?;
            // 调用 ServiceRequest，得到 ServiceResponse
            let res = svc.call(req).await;
            res
        })
    }
}
