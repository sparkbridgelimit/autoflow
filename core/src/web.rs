use crate::resource::Resource;

pub fn resource(path: &str) -> Resource {
  Resource::new(path)
}