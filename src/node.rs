use crate::{
    endpoint::EndpointRef,
    enums::{NodeType, Status},
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 表示节点在画布上的位置。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// 定义端点配置的结构体，用于表示输入和输出
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndpointConfig {
    pub id: String,           // 输入或输出端点的唯一标识符
    pub name: String,         // 输入或输出端点名称
    pub required: bool,       // 是否为必需连接的输入
    pub data_type: String,    // 输入数据类型，如"arrow", "json"
    pub display_type: String, // 用于UI显示的类型
    pub description: String,  // 输入描述
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub node_type: String,
    pub name: String,
    pub description: String,
    pub inputs: Vec<EndpointConfig>,
    pub outputs: Vec<EndpointConfig>,
    pub data_schema: serde_json::Value,
    pub data: serde_json::Value,
    pub data_ui_schema: serde_json::Value,
    pub component: String,
    pub executor_id: String,
    pub status: String,
    pub extra: Option<ExtraConfig>,
}

impl Node {
    pub fn new(name: String, node_type: String) -> Self {
        Node {
            id: nanoid!(8),
            node_type,
            name,
            description: String::default(),
            inputs: vec![],
            outputs: vec![],
            data_schema: serde_json::Value::default(),
            data: serde_json::Value::default(),
            data_ui_schema: serde_json::Value::default(),
            component: String::default(),
            executor_id: String::default(),
            status: Status::Pending.code().to_string(),
            extra: None,
        }
    }
}

pub trait NodeBuilderTrait {
    fn start(name: &str) -> Self;
    fn normal(name: &str) -> Self;
    fn add_input_endpoint(&mut self);
    fn add_output_endpoint(&mut self);
}

impl NodeBuilderTrait for Node {
    fn start(name: &str) -> Self {
        let ed = EndpointConfig {
            id: nanoid!(8),
            name: "output".to_string(),
            required: true,
            data_type: String::default(),
            display_type: String::default(),
            description: String::default(),
        };

        let mut outputs = vec![];
        outputs.push(ed);

        Node {
            id: nanoid!(8),
            node_type: NodeType::Start.code().to_string(),
            name: name.to_string(),
            description: String::default(),
            inputs: vec![],
            outputs: vec![],
            data_schema: serde_json::Value::default(),
            data: serde_json::Value::default(),
            data_ui_schema: serde_json::Value::default(),
            component: String::default(),
            executor_id: String::default(),
            status: Status::Pending.code().to_string(),
            extra: None,
        }
    }

    fn normal(name: &str) -> Self {
        let ed = EndpointConfig {
            id: nanoid!(8),
            name: "output".to_string(),
            required: true,
            data_type: String::default(),
            display_type: String::default(),
            description: String::default(),
        };

        let mut outputs = vec![];
        outputs.push(ed);

        Node {
            id: nanoid!(8),
            node_type: NodeType::Normal.code().to_string(),
            name: name.to_string(),
            description: String::default(),
            inputs: vec![],
            outputs: vec![],
            data_schema: serde_json::Value::default(),
            data: serde_json::Value::default(),
            data_ui_schema: serde_json::Value::default(),
            component: String::default(),
            executor_id: String::default(),
            status: Status::Pending.code().to_string(),
            extra: None,
        }
    }

    fn add_input_endpoint(&mut self) {
        let len = self.inputs.len();
        self.inputs.push(EndpointConfig {
            id: "input_b".to_string(),
            name: format!("input-{}", len + 1),
            required: true,
            data_type: "json".to_string(),
            display_type: "text".to_string(),
            description: "Input of node B".to_string(),
        });
    }

    fn add_output_endpoint(&mut self) {
        let len = self.outputs.len();
        self.outputs.push(EndpointConfig {
            id: nanoid!(8),
            name: format!("output-{}", len + 1),
            required: true,
            data_type: "json".to_string(),
            display_type: "text".to_string(),
            description: "Output of node B".to_string(),
        });
    }
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

pub trait NodeAttrTrait {
    fn get_input_ref(&self, index: i32) -> Option<EndpointRef>;
    fn get_output_ref(&self, index: i32) -> Option<EndpointRef>;
    fn get_input(&self, index: i32) -> Option<&EndpointConfig>;
    fn get_output(&self, index: i32) -> Option<&EndpointConfig>;
}

impl NodeAttrTrait for Node {
    // 获取输入端点
    fn get_input(&self, index: i32) -> Option<&EndpointConfig> {
        if index >= 0 && (index as usize) < self.inputs.len() {
            Some(&self.inputs[index as usize])
        } else {
            None
        }
    }

    // 获取输出端点
    fn get_output(&self, index: i32) -> Option<&EndpointConfig> {
        if index >= 0 && (index as usize) < self.outputs.len() {
            Some(&self.outputs[index as usize])
        } else {
            None
        }
    }

    // 获取输入端点引用
    fn get_input_ref(&self, index: i32) -> Option<EndpointRef> {
        if index >= 0 && (index as usize) < self.inputs.len() {
            let endpoint_config = &self.inputs[index as usize];
            Some(EndpointRef {
                node_id: self.id.clone(),
                endpoint_id: endpoint_config.id.clone(),
            })
        } else {
            None
        }
    }

    // 获取输出端点引用
    fn get_output_ref(&self, index: i32) -> Option<EndpointRef> {
        if index >= 0 && (index as usize) < self.outputs.len() {
            let endpoint_config = &self.outputs[index as usize];
            Some(EndpointRef {
                node_id: self.id.clone(),
                endpoint_id: endpoint_config.id.clone(),
            })
        } else {
            None
        }
    }
}
