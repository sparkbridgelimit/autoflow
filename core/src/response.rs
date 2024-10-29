use crate::error::Error;

pub struct TaskResponse {
    pub message: String
}

impl TaskResponse {
    #[inline]
    pub fn from_error(_error: impl Into<Error>) -> Self {
        Self {
            message: String::default()
        }
    }
}
