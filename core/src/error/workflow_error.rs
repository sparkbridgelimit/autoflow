use std::{
    convert::Infallible,
    error::Error as StdError,
    fmt,
};
use super::{downcast_dyn, downcast_get_type_id, status::StatusCode};

pub trait WorkflowError: fmt::Debug + fmt::Display {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    downcast_get_type_id!();
}

downcast_dyn!(WorkflowError);

impl WorkflowError for Box<dyn StdError + 'static> {}

impl WorkflowError for Infallible {
    fn status_code(&self) -> StatusCode {
        match *self {}
    }
}

