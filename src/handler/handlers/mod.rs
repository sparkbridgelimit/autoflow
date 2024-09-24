pub mod start;
pub mod log;
pub mod end;

use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use start::StartHandler;
use log::LogHandler;
use end::EndHandler;

use super::handler_trait::NodeHandlerTrait;

pub fn load_handlers() -> Vec<Box<dyn NodeHandlerTrait>> {
  vec![
      Box::new(StartHandler),
      Box::new(LogHandler),
      Box::new(EndHandler),
  ]
}

// 使用 once_cell::sync::Lazy 来定义静态的 handlers_map，使用 Arc 包装 Box
pub static HANDLERS_MAP: Lazy<HashMap<&'static str, Arc<dyn NodeHandlerTrait + Send + Sync>>> = Lazy::new(|| {
  let mut map: HashMap<&'static str, Arc<dyn NodeHandlerTrait + Send + Sync>> = HashMap::new();
  
  // 初始化 handlers_map
  map.insert("start", Arc::new(StartHandler));
  map.insert("log", Arc::new(LogHandler));
  map.insert("end", Arc::new(EndHandler));
  
  map
});

// 提供便捷方法获取 handlers_map
pub fn get_handlers_map() -> &'static HashMap<&'static str, Arc<dyn NodeHandlerTrait + Send + Sync>> {
  &HANDLERS_MAP
}