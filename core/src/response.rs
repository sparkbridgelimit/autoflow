use crate::error::Error;

pub struct TaskResponse {}

impl TaskResponse {
    #[inline]
    pub fn from_error(error: impl Into<Error>) -> Self {
        Self {}
    }
}
