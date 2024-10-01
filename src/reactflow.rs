use crate::{node::Position, viewport::ViewPort};

pub struct ReactFlowNode {
  pub id: String,          // 节点唯一标识符
  pub type_: String,       // 节点类型
  pub position: Position,  // 节点在画布上的位置
  pub data: serde_json::Value,  // 节点的数据
}

pub struct ReactFlowEdge {
  pub id: String,          // 边的唯一标识符
  pub source: String,      // 边的起始节点ID
  pub source_handle: String, // 边的起始端点ID
  pub target: String,      // 边的目标节点ID
  pub target_handle: String, // 边的目标端点ID
}

pub struct ReactFlow {
  pub nodes: Vec<ReactFlowNode>,
  pub edges: Vec<ReactFlowEdge>,
  pub viewport: ViewPort,  // 视口
  pub zoom: i32,           // 缩放比例
}
