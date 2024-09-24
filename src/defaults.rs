use serde_json::Value;
use nanoid::nanoid;

use crate::{enums::{NodeType, Status}, node::{EndpointConfig, ExtraConfig}};

pub fn generate_id() -> String {
    nanoid!(8)
}

// 默认 node_type 为 Normal
pub fn default_node_type() -> NodeType {
    NodeType::Normal
}

// 默认 description
pub fn default_description() -> String {
    "No description available".to_string()
}

// 默认 implementation
pub fn default_implementation() -> String {
    "default_implementation".to_string()
}

// 默认 inputs
pub fn default_inputs() -> Vec<EndpointConfig> {
    vec![]
}

// 默认 outputs
pub fn default_outputs() -> Vec<EndpointConfig> {
    vec![]
}

// 默认 data_schema
pub fn default_data_schema() -> Value {
    serde_json::json!({})
}

// 默认 data
pub fn default_data() -> Value {
    serde_json::json!({})
}

// 默认 data_ui_schema
pub fn default_data_ui_schema() -> Value {
    serde_json::json!({})
}

// 默认 component
pub fn default_component() -> String {
    "default_component".to_string()
}

// 默认 execute
pub fn default_execute() -> String {
    "default_execute".to_string()
}

pub fn default_status() -> Status {
    Status::Pending
}

// 默认 extra
pub fn default_extra() -> Option<ExtraConfig> {
    None
}
