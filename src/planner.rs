use crate::{edge::Edge, node::Node};

pub struct Planner {
  pub nodes: Vec<Node>,
  pub edges: Vec<Edge>,
  pub node_map: std::collections::HashMap<String, Node>, // 方便查询的节点映射
  pub edge_map: std::collections::HashMap<String, Vec<Edge>>, // 方便查询的边映射
}

impl Planner {
  pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
      let node_map = nodes
          .iter()
          .map(|node| (node.id.clone(), node.clone()))
          .collect::<std::collections::HashMap<_, _>>();

      let mut edge_map: std::collections::HashMap<String, Vec<Edge>> = std::collections::HashMap::new();
      for edge in &edges {
          edge_map
              .entry(edge.source.node_id.clone())
              .or_insert(Vec::new())
              .push(edge.clone());
      }

      Planner {
          nodes,
          edges,
          node_map,
          edge_map,
      }
  }
}

pub trait PlannerTrait {
  fn find_start_node(&self) -> Option<&Node>;
  fn find_end_node(&self) -> Option<&Node>;
  fn next_task(&self, current_node_id: &str, node_data: &str) -> Vec<NodePlan>;
}
