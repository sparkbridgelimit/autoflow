use custom::CustomTaskHandler;
use end::EndTaskHandler;
use once_cell::sync::Lazy;
use start::StartTaskHandler;

use crate::handler2::TaskHandler;


pub mod custom;
pub mod start;
pub mod end;


// 使用 Lazy 初始化静态的 HANDLERS
pub static HANDLERS: Lazy<Vec<Box<dyn TaskHandler>>> = Lazy::new(|| {
  vec![
      Box::new(StartTaskHandler {}),
      Box::new(CustomTaskHandler {}),
      Box::new(EndTaskHandler {}),
  ]
});