use serde::{Deserialize, Serialize};

// 定义端点（Endpoint）结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Endpoint {
    pub node_id: String, // 节点ID
    pub port_id: String, // 端点ID（输入或输出端点）
}
