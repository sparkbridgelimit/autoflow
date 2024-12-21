use crate::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// 节点执行状态
#[derive(Debug, Clone)]
pub enum TaskStatus {
    Success,
    Failure,
    Pending,
    Running,
    Canceled,
    // 可以再加其他状态，如 PartiallySucceeded, Skipped 等
}

/// 节点执行成功时的产出数据
#[derive(Debug, Clone)]
pub struct TaskResult {
    /// 可以是任何结构化数据，这里用 serde_json::Value 以便灵活存放任意 JSON
    pub data: serde_json::Value,
}

/// 节点执行失败时的错误信息
#[derive(Debug, Clone)]
pub struct TaskError {
    pub code: String,      // 自定义错误码，如 "INVALID_PARAM", "TIMEOUT" 等
    pub message: String,   // 错误描述信息
}

/// 通用的 TaskResponse 结构体
///
/// 适用于工作流场景，涵盖执行状态、结果、错误和元信息（节点名、时间戳等）。
#[derive(Debug, Clone)]
pub struct TaskResponse {
    pub status: TaskStatus,         // 执行状态
    pub result: Option<TaskResult>, // 如果成功或部分成功才有
    pub error: Option<TaskError>,   // 如果失败才有
    pub node_name: Option<String>,  // 节点名称或 ID
    pub timestamp: Option<u64>,     // 时间戳（毫秒），可用来记录开始/结束时间或耗时
    // 如有需要，可再加更多字段
}

impl TaskResponse {
    pub fn empty() -> Self {
        Self {
            status: TaskStatus::Pending,
            result: None,
            error: None,
            node_name: None,
            timestamp: None,
        }
    }

    /// 构造一个成功的 TaskResponse
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            status: TaskStatus::Success,
            result: Some(TaskResult { data }),
            error: None,
            node_name: None,
            timestamp: Some(now_millis()),
        }
    }

    /// 构造一个失败的 TaskResponse
    pub fn fail(code: &str, message: &str) -> Self {
        Self {
            status: TaskStatus::Failure,
            result: None,
            error: Some(TaskError {
                code: code.to_string(),
                message: message.to_string(),
            }),
            node_name: None,
            timestamp: Some(now_millis()),
        }
    }

    /// 从引擎内部的 Error 生成失败的 TaskResponse
    pub fn from_error(e: Error) -> Self {
        // 根据你的 Error 类型，提取更具体的 code 和 message
        Self {
            status: TaskStatus::Failure,
            result: None,
            error: Some(TaskError {
                code: "ENGINE_ERROR".to_string(),
                message: format!("{}", e), // 或者 e.to_string()
            }),
            node_name: None,
            timestamp: Some(now_millis()),
        }
    }

    /// 设置节点名称
    pub fn with_node_name(mut self, node_name: impl Into<String>) -> Self {
        self.node_name = Some(node_name.into());
        self
    }

    /// 设置自定义时间戳
    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp = Some(ts);
        self
    }
}

/// 获取当前系统时间（毫秒）
/// 这里简单演示，你也可以改成自己常用的时间库，如 chrono。
fn now_millis() -> u64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    dur.as_millis() as u64
}