use std::rc::Rc;

use crate::{router::ResourceDef, service::BoxedTaskServiceFactory};

pub struct TaskRegistry {
  tasks: Vec<(ResourceDef, Rc<BoxedTaskServiceFactory>)>,
  default: Rc<BoxedTaskServiceFactory>,
}

impl TaskRegistry {
  pub fn new(default: Rc<BoxedTaskServiceFactory>) -> Self {
      Self {
          tasks: Vec::new(),
          default,
      }
  }

  pub fn insert(&mut self, rdef: ResourceDef, factory: Rc<BoxedTaskServiceFactory>) {
      self.tasks.push((rdef, factory));
  }

  pub fn get_factory(&self, task_name: &str) -> &Rc<BoxedTaskServiceFactory> {
      for (rdef, fac) in &self.tasks {
          if rdef.is_match(task_name) {
              return fac;
          }
      }
      &self.default
  }
}
