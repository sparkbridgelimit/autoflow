use std::sync::Arc;

use custom::CustomTaskHandler;
use end::EndTaskHandler;
use once_cell::sync::Lazy;
use start::StartTaskHandler;

use crate::handler2::TaskHandler;

pub mod custom;
pub mod end;
pub mod start;

// 使用 Lazy 初始化静态的 HANDLERS
pub static HANDLERS: Lazy<Vec<Arc<dyn TaskHandler + Send + Sync>>> = Lazy::new(|| {
    vec![
        Arc::new(StartTaskHandler {}),
        Arc::new(CustomTaskHandler {}),
        Arc::new(EndTaskHandler {}),
    ]
});
