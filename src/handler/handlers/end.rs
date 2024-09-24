use crate::handler::handler_trait::NodeHandlerTrait;

pub struct EndHandler;

impl NodeHandlerTrait for EndHandler {
    fn node_id(&self) -> &str {
        "end"
    }

    fn execute(&self) {
        println!("Executing End Node logic");
        // 这里可以放置具体的 End 节点逻辑
    }
}
