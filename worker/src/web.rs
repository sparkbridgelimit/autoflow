use crate::resource::Resource;

pub fn resource(task_name: &str) -> Resource {
  Resource::new(task_name)
}