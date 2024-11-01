use serde::{Deserialize, Serialize};

// 定义端点（Endpoint）结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndpointRef {
    pub node_id: String,     // 节点ID
    pub endpoint_id: String, // 端点ID（输入或输出端点）
}

impl EndpointRef {
    pub fn new(node_id: &str, endpoint_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            endpoint_id: endpoint_id.to_string(),
        }
    }
}
