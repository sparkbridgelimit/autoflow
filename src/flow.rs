use crate::{edge::Edge, node::Node, viewport::ViewPort};

pub struct Flow {
  pub edge: Edge,
  pub node: Node,
  pub viewport: ViewPort,
  pub zoom: i32
}