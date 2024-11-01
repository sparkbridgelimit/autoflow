use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Start,
    Normal,
    End
}

impl NodeType {
    // 自定义方法，返回对应的字符串值
    pub fn code(&self) -> String {
        match self {
            NodeType::Start => "start".to_string(),
            NodeType::Normal => "normal".to_string(),
            NodeType::End => "end".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pending,
    Running,
    Success,
    Failed,
}

impl Status {
    // 自定义方法，返回对应的字符串值
    pub fn code(&self) -> String {
        match self {
            Status::Pending => "pending".to_string(),
            Status::Running => "running".to_string(),
            Status::Success => "success".to_string(),
            Status::Failed => "failed".to_string(),
        }
    }
}