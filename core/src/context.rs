use std::rc::Rc;

pub struct TaskContext {
  pub(crate) inner: Rc<TaskContextInner>
}

pub (crate) struct TaskContextInner {}