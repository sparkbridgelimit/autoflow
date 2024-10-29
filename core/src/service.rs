use actix_service::{
    boxed::{BoxService, BoxServiceFactory},
    IntoServiceFactory, ServiceFactory,
};

use crate::{
    config::AppService,
    context::TaskContext,
    error::Error,
    extractor::FromContext,
    response::TaskResponse, router::ResourceDef,
};

pub(crate) type BoxedTaskService = BoxService<ServiceRequest, ServiceResponse, Error>;
pub(crate) type BoxedTaskServiceFactory =
    BoxServiceFactory<(), ServiceRequest, ServiceResponse, Error, ()>;

pub trait HttpServiceFactory {
    fn register(self, config: &mut AppService);
}

impl<T: HttpServiceFactory> HttpServiceFactory for Vec<T> {
    fn register(self, config: &mut AppService) {
        self.into_iter()
            .for_each(|factory| factory.register(config));
    }
}

pub(crate) trait AppServiceFactory {
    fn register(&mut self, config: &mut AppService);
}

pub(crate) struct ServiceFactoryWrapper<T> {
    factory: Option<T>,
}

impl<T> ServiceFactoryWrapper<T> {
    pub fn new(factory: T) -> Self {
        Self {
            factory: Some(factory),
        }
    }
}

impl<T> AppServiceFactory for ServiceFactoryWrapper<T>
where
    T: HttpServiceFactory,
{
    fn register(&mut self, config: &mut AppService) {
        if let Some(item) = self.factory.take() {
            item.register(config)
        }
    }
}

pub struct ServiceRequest {
    ctx: TaskContext,
}

impl ServiceRequest {
    pub(crate) fn new(ctx: TaskContext) -> Self {
        Self { ctx }
    }

    #[inline]
    pub fn into_parts(self) -> TaskContext {
        self.ctx
    }

    #[inline]
    pub fn ctx(&self) -> &TaskContext {
        &self.ctx
    }

    #[inline]
    pub fn ctx_mut(&mut self) -> &mut TaskContext {
        &mut self.ctx
    }

    pub fn extract<T>(&mut self) -> <T as FromContext>::Future
    where
        T: FromContext,
    {
        T::from_context(&self.ctx)
    }

    #[inline]
    pub fn from_context(ctx: TaskContext) -> Self {
        ServiceRequest { ctx }
    }

    #[inline]
    pub fn into_response<R: Into<TaskResponse>>(self, res: R) -> ServiceResponse {
        let res = TaskResponse::from(res.into());
        ServiceResponse::new(self.ctx, res)
    }

    #[inline]
    pub fn error_response<E: Into<Error>>(self, err: E) -> ServiceResponse {
        let res = TaskResponse::from_error(err.into());
        ServiceResponse::new(self.ctx, res)
    }
}

pub struct ServiceResponse {
    ctx: TaskContext,
    res: TaskResponse,
}

impl ServiceResponse {
    pub fn new(ctx: TaskContext, res: TaskResponse) -> Self {
        ServiceResponse { ctx, res }
    }

    #[inline]
    pub fn request(&self) -> &TaskContext {
        &self.ctx
    }

    #[inline]
    pub fn response(&self) -> &TaskResponse {
        &self.res
    }

    #[inline]
    pub fn response_mut(&mut self) -> &mut TaskResponse {
        &mut self.res
    }

    #[inline]
    pub fn into_parts(self) -> (TaskContext, TaskResponse) {
        (self.ctx, self.res)
    }
}

impl From<ServiceResponse> for TaskResponse {
    fn from(res: ServiceResponse) -> TaskResponse {
        res.res
    }
}

pub struct WebService {
    name: Option<String>,
}

impl WebService {
    pub fn new() -> Self {
        WebService { name: None }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn finish<T, F>(self, service: F) -> impl HttpServiceFactory
    where
        F: IntoServiceFactory<T, ServiceRequest>,
        T: ServiceFactory<
                ServiceRequest,
                Config = (),
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            > + 'static,
    {
        WebServiceImpl {
            srv: service.into_factory(),
            name: self.name,
        }
    }
}

struct WebServiceImpl<T> {
    srv: T,
    name: Option<String>,
}

impl<T> HttpServiceFactory for WebServiceImpl<T>
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

        config.register_service(rdef, self.srv)
    }
}

#[macro_export]
macro_rules! services {
  () => {()};
  ($($x:expr),+ $(,)?) => {
      ($($x,)+)
  }
}

macro_rules! service_tuple ({ $($T:ident)+ } => {
  impl<$($T: HttpServiceFactory),+> HttpServiceFactory for ($($T,)+) {
      #[allow(non_snake_case)]
      fn register(self, config: &mut AppService) {
          let ($($T,)*) = self;
          $($T.register(config);)+
      }
  }
});

service_tuple! { A }
service_tuple! { A B }
service_tuple! { A B C }
service_tuple! { A B C D }
service_tuple! { A B C D E }
service_tuple! { A B C D E F }
service_tuple! { A B C D E F G }
service_tuple! { A B C D E F G H }
service_tuple! { A B C D E F G H I }
service_tuple! { A B C D E F G H I J }
service_tuple! { A B C D E F G H I J K }
service_tuple! { A B C D E F G H I J K L }
