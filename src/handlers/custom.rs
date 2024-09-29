use crate::handler2::TaskHandler;

pub struct CustomTaskHandler {}

impl TaskHandler for CustomTaskHandler {
  
    fn before(&self) {
        println!("开始处理任务");
    }

    fn after(&self) {
        println!("完成任务");
    }

    fn handle(&self) {
        println!("正在处理任务数据");
    }
    
    fn for_task (&self) -> &'static str {
        "custom"
    }
}
