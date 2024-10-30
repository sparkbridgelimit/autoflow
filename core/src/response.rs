use crate::error::Error;

pub struct TaskResponse {
    pub message: String
}

impl TaskResponse {
    #[inline]
    pub fn new() -> Self {
        Self {
            message: String::default()
        }
    }
}

impl TaskResponse {
    #[inline]
    pub fn from_error(_error: impl Into<Error>) -> Self {
        Self {
            message: String::default()
        }
    }
}

impl std::fmt::Debug for TaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskResponse")
            .field("message", &self.message)
            .finish()
    }
}