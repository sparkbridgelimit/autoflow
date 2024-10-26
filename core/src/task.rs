use serde::{Deserialize, Serialize};

// 任务状态的枚举类型
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum TaskStatus {
    Queued,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Task {
    pub task_type: String,
    // 当前任务要处理的数据
    pub data: serde_json::Value,
    // 当前任务状态
    pub status: TaskStatus,
}

impl Task {
    // 新建任务的构造函数
    pub fn new(task_type: String, data: serde_json::Value) -> Self {
        Task {
            task_type,
            data,
            status: TaskStatus::Queued,
        }
    }
}
