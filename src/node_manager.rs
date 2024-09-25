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

    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // 从 JSON 文件加载节点
        let nodes = NodeManager::load_nodes_from_json("nodes.json")?;

        // 使用静态 handlers_map 初始化 NodeManager
        Ok(NodeManager {
            nodes,
            handlers_map: get_handlers_map().clone(),
        })
    }

    fn load_nodes_from_json(file_path: &str) -> Result<Vec<Node>, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let nodes: Vec<Node> = serde_json::from_reader(reader)?;
        Ok(nodes)
    }

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

