use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{defaults::*, enums::{NodeType, Status}};

/// 表示节点在画布上的位置。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// 定义端点配置的结构体，用于表示输入和输出
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndpointConfig {
    pub id: String,            // 输入或输出端点的唯一标识符
    pub name: String,          // 输入或输出端点名称
    pub required: bool,        // 是否为必需连接的输入
    pub endpoint_type: String, // 输入数据类型，如"arrow", "json"
    pub display_type: String,  // 用于UI显示的类型
    pub description: String,   // 输入描述
}

// 定义扩展字段配置（如重试策略）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtraConfig {
    pub retry: RetryConfig, // 重试策略配置
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryConfig {
    pub max_attempts: u32, // 最大重试次数
    pub delay: u32,        // 重试延迟时间（秒）
}

#[derive(Debug, Clone, Deserialize, Serialize)] // 添加 Deserialize 和 Serialize
pub struct Node {
    #[serde(default = "generate_id")] 
    pub id: String,
    
    #[serde(default = "default_node_type")]
    pub node_type: NodeType,
    
    #[serde(default)]
    pub name: String,
    
    #[serde(default)]
    pub description: String,
        
    #[serde(default)]
    pub inputs: Vec<EndpointConfig>,
    
    #[serde(default)]
    pub outputs: Vec<EndpointConfig>,
    
    #[serde(default)]
    pub data_schema: serde_json::Value,
    
    #[serde(default)]
    pub data: serde_json::Value,
    
    #[serde(default)]
    pub data_ui_schema: serde_json::Value,
    
    #[serde(default)]
    pub component: String,
    
    #[serde(default)]
    pub exec_fun: String,
    
    #[serde(default = "default_status")]
    pub status: Status,
    
    #[serde(default)]
    pub extra: Option<ExtraConfig>,
}

pub trait NodeTrait {
    fn from_json(json: &Value) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

impl NodeTrait for Node {
    fn from_json(json: &Value) -> Result<Self, Box<dyn std::error::Error>> {
        let node: Node = serde_json::from_value(json.clone())?;
        Ok(node)
    }
}

