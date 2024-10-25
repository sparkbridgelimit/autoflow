use actix::Actor;
use actix::prelude::*;

/// 工作流实例
pub struct Workflow {}

/// Actix actor implementation for Workflow.
impl Actor for Workflow {
  type Context = Context<Self>;
}

/// Message type for receive state from related worker.
pub struct WorkflowMessage {
  // 任务id
  // 状态
  // 数据
  // 错误
}


impl Message for WorkflowMessage {
    type Result = ();
}