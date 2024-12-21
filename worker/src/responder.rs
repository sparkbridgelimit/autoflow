use crate::{context::TaskContext, error::Error, response::TaskResponse};

pub trait Responder {
  type Output;

  fn respond_to(self, ctx: &TaskContext) -> TaskResponse;
}

impl<T> Responder for Result<T, Error>
where
    T: Send + 'static + std::fmt::Display,
{
    type Output = T;
    
    fn respond_to(self, _ctx: &TaskContext) -> TaskResponse {
        TaskResponse::from(self)
    }
}

impl<T> From<Result<T, Error>> for TaskResponse
where
    T: std::fmt::Display,
{
    fn from(res: Result<T, Error>) -> Self {
        match res {
            Ok(val) => {
                TaskResponse::success(serde_json::json!(val.to_string()))
            }
            Err(e) => {
                TaskResponse::from_error(e)
            }
        }
    }
}
