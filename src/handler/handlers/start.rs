use crate::handler::handler_trait::NodeHandlerTrait;

pub struct StartHandler;

impl NodeHandlerTrait for StartHandler {
    fn node_id(&self) -> &str {
        "start"
    }

    fn execute(&self) {
        println!("Executing Start Node logic");
        // 这里可以放置具体的 Start 节点逻辑
    }
}
