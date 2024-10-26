use std::collections::{HashMap, HashSet};

use crate::{edge::Edge, node::Node};

/// A planner for executing workflows, determining the next executable nodes
/// based on defined edges and each node's requirements.
pub struct WorkflowPlanner {
    /// List of all nodes in the workflow.
    pub nodes: Vec<Node>,
    /// List of all edges in the workflow, connecting nodes.
    pub edges: Vec<Edge>,
    /// A mapping of node IDs to their corresponding nodes.
    pub node_map: HashMap<String, Node>,
    /// A mapping of source node IDs to their outgoing edges.
    pub edge_map: HashMap<String, Vec<Edge>>,
    /// A set of IDs of nodes that have been visited or executed.
    pub visited: HashSet<String>,
    /// The predefined start node for the workflow.
    pub start_node: Node,
}

impl WorkflowPlanner {
    /// Creates a new `WorkflowPlanner` instance with nodes and edges, establishing
    /// the node and edge mappings and identifying the unique start node.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A vector of `Node` instances representing each node in the workflow.
    /// * `edges` - A vector of `Edge` instances representing connections between nodes.
    ///
    /// # Panics
    ///
    /// Panics if more than one start node is found, or if no start node is defined.
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        let node_map = nodes
            .iter()
            .map(|node| (node.id.clone(), node.clone()))
            .collect::<std::collections::HashMap<_, _>>();

        let mut edge_map: HashMap<String, Vec<Edge>> = HashMap::new();
        for edge in &edges {
            edge_map
                .entry(edge.source.node_id.clone())
                .or_default()
                .push(edge.clone());
        }

        // 查找并设置唯一的起始节点
        let start_node = Self::find_start_node(&nodes)
            .expect("Start node not found") // 如果没有找到起始节点，则抛出错误
            .clone();

        WorkflowPlanner {
            nodes,
            edges,
            node_map,
            edge_map,
            visited: HashSet::new(),
            start_node,
        }
    }

    /// Finds the start node of type `"start"` from the list of nodes.
    ///
    /// Returns an option containing the reference to the start node if found,
    /// or `None` if no start node is present. Panics if multiple start nodes exist.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A slice of nodes to search for the start node.
    fn find_start_node(nodes: &[Node]) -> Option<&Node> {
        let mut start_nodes = nodes
            .iter()
            .filter(|node| node.node_type == "start")
            .collect::<Vec<_>>();

        match start_nodes.len() {
            1 => Some(start_nodes.remove(0)),
            0 => None,
            _ => panic!("Multiple start nodes found. Ensure only one start node is defined."),
        }
    }

    /// Determines the next executable nodes from a given node based on the workflow's edges.
    ///
    /// This function examines the outgoing edges from the specified `node` and identifies
    /// the next nodes that can be executed. It checks each target node's input requirements
    /// to ensure they are satisfied before adding them to the result.
    ///
    /// # Arguments
    ///
    /// * `node` - A reference to the current node to evaluate.
    /// * `_data` - A reference to additional data in `serde_json::Value` format (unused).
    ///
    /// # Returns
    ///
    /// Returns a vector of `Node` instances that can be executed next.
    /// Each node in the result has all required inputs met and is not already visited.
    pub fn next_nodes(&mut self, node: &Node, _data: &serde_json::Value) -> Vec<Node> {
        self.visited.insert(node.id.clone());

        let mut next_nodes = Vec::new();
        let mut seen_nodes = HashSet::new();

        // Find all outgoing edges for the current node
        let outgoing_edges = self
            .edges
            .iter()
            .filter(|edge| edge.source.node_id == node.id);

        // Check each target node from the outgoing edges
        for edge in outgoing_edges {
            let target_node_id = &edge.target.node_id;

            // Skip nodes that have already been visited or processed
            if !self.visited.contains(target_node_id) && !seen_nodes.contains(target_node_id) {
                // Retrieve the target node
                if let Some(target_node) = self.node_map.get(target_node_id) {
                    // Ensure all required inputs of the target node are satisfied
                    if self.is_node_ready(target_node) {
                        next_nodes.push(target_node.clone());
                        seen_nodes.insert(target_node_id.clone());
                    }
                } else {
                    eprintln!(
                        "Warning: Target node {} not found in node map",
                        target_node_id
                    );
                }
            }
        }
        next_nodes
    }

    /// Checks if all required inputs of a node are satisfied for execution.
    ///
    /// # Arguments
    ///
    /// * `target_node` - A reference to the target `Node` to check for input readiness.
    ///
    /// # Returns
    ///
    /// Returns `true` if all required inputs are met, otherwise `false`.
    fn is_node_ready(&self, target_node: &Node) -> bool {
        target_node.inputs.iter().all(|input_endpoint| {
            if !input_endpoint.required {
                return true;
            }

            self.edges.iter().any(|edge| {
                edge.target.node_id == target_node.id
                    && edge.target.endpoint_id == input_endpoint.id
                    && self.visited.contains(&edge.source.node_id)
            })
        })
    }
}
