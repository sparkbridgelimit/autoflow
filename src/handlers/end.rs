use async_trait::async_trait;

use crate::handler2::TaskHandler;

pub struct EndTaskHandler {}

#[async_trait]
impl TaskHandler for EndTaskHandler {
  
    async fn before(&self) {
        println!("开始处理任务");
    }

    async fn after(&self) {
        println!("完成任务");
    }

    async fn handle(&self) {
        println!("正在处理任务数据");
    }
    
    fn for_task (&self) -> &'static str {
        "custom"
    }
}
