use serde::{Deserialize, Serialize};

use crate::{edge::Edge, node::Node, viewport::ViewPort};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Flow {
  pub edges: Vec<Edge>,
  pub nodes: Vec<Node>,
  pub viewport: ViewPort,
  pub zoom: i32
}