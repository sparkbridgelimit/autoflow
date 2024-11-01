use std::rc::Rc;

pub struct TaskContext {
    pub(crate) inner: Rc<TaskContextInner>,
}

pub(crate) struct TaskContextInner {
    pub(crate) name: String,
}

impl TaskContext {
    #[inline]
    pub fn name(&self) -> &str {
        &self.inner.name
    }
}

impl TaskContext {
    #[inline]
    pub(crate) fn new(name: &str) -> TaskContext {
        TaskContext {
            inner: Rc::new(TaskContextInner { name: name.to_owned() }),
        }
    }
}
