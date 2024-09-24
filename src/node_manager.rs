use crate::handler::handler_trait::NodeHandlerTrait;
use crate::handler::handlers::get_handlers_map;
use crate::node::Node;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error};
use std::sync::Arc;

pub struct NodeManager {
    pub nodes: Vec<Node>,
    pub handlers_map: HashMap<&'static str, Arc<dyn NodeHandlerTrait + Send + Sync>>, // 使用 Arc 而不是 Box
}

impl NodeManager {
    // 初始化一个空的 NodeManager
    pub fn new() -> Self {
        NodeManager {
            nodes: Vec::new(),
            handlers_map: HashMap::new(),
        }
    }

    // 返回一个 Result<Self, Error>，从 JSON 文件初始化节点
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // 从 JSON 文件加载节点
        let nodes = NodeManager::load_nodes_from_json("nodes.json")?;

        // 使用静态 handlers_map 初始化 NodeManager
        Ok(NodeManager {
            nodes,
            handlers_map: get_handlers_map().clone(),
        })
    }

    // 辅助函数：从 JSON 文件加载节点
    fn load_nodes_from_json(file_path: &str) -> Result<Vec<Node>, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let nodes: Vec<Node> = serde_json::from_reader(reader)?;
        Ok(nodes)
    }

    // 执行节点的 handler
    pub fn run(&self, node: &Node) -> Result<()> {
        let handler = match self.handlers_map.get(node.id.as_str()) {
            Some(handler) => handler,
            None => return Err(anyhow::anyhow!("Handler for node {} not found", node.id)),
        };

        println!("Starting execution of node: {}", node.id);
        // 自定义逻辑运行
        handler.execute();
        // 后置逻辑处理
        println!("Finished execution of node: {}", node.id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::node::NodeTrait;

    use super::*;
    use std::sync::Arc;

    struct TestHandler;

    impl NodeHandlerTrait for TestHandler {
        fn node_id(&self) -> &str {
            "test"
        }

        fn execute(&self) {
            println!("TestHandler executed.");
        }
    }

    #[test]
    fn test_run_success() {
        // 初始化 NodeManager，并添加一个测试节点
        let mut manager = NodeManager::new();

        // 添加测试节点
        let json_data = json!({
            "id": "start",
            "node_type": "start",
            "name": "Start Node",
            "description": "This is the start node of the workflow.",
            "implementation": "start_function",
            "inputs": [],
            "outputs": [
                {
                    "id": "output_001",
                    "name": "Start Output",
                    "required": false,
                    "endpoint_type": "arrow",
                    "display_type": "single_value",
                    "description": "The output of the start node"
                }
            ],
            "data_schema": {},
            "data": {},
            "data_ui_schema": {},
            "component": "start_component",
            "execute": "start_logic",
            "status": "PENDING",
            "extra": null
        });

        let node = Node::from_json(&json_data).unwrap();
        
        manager.nodes.push(node.clone());

        // 添加对应的 handler
        manager.handlers_map.insert("test", Arc::new(TestHandler));

        // 执行节点
        let result = manager.run(&node);

        // 断言运行成功
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_handler_not_found() {
        // 初始化 NodeManager，但不添加 handler
        let mut manager = NodeManager::new();

        // 添加测试节点
        let node = Node {
            id: "nonexistent".to_string(),
            node_type: "action".to_string(),
        };
        manager.nodes.push(node.clone());

        // 尝试执行不存在的 handler
        let result = manager.run(&node);

        // 断言运行失败
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Handler for node nonexistent not found");
        }
    }
}
