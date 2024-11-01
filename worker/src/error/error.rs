use std::{error::Error as StdError, fmt};

use super::workflow_error::WorkflowError;


pub struct Error {
    cause: Box<dyn WorkflowError>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.cause, f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.cause)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<T: WorkflowError + 'static> From<T> for Error {
    fn from(err: T) -> Error {
        Error {
            cause: Box::new(err),
        }
    }
}

impl From<Box<dyn WorkflowError>> for Error {
    fn from(value: Box<dyn WorkflowError>) -> Self {
        Error { cause: value }
    }
}
