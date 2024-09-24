use crate::handler::handler_trait::NodeHandlerTrait;

pub struct LogHandler;

impl NodeHandlerTrait for LogHandler {
    fn node_id(&self) -> &str {
        "log"
    }

    fn execute(&self) {
        println!("Executing Log Node logic");
        // 这里可以放置具体的 Log 节点逻辑
    }
}
