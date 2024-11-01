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

#[cfg(test)]
mod tests {
    use crate::endpoint::EndpointRef;
    use crate::node::EndpointConfig;
    use crate::node::Position;

    use super::*;
    use serde_json::json;

    // Helper function to create a simple node
    fn create_node(id: &str, node_type: &str, required_inputs: Vec<EndpointConfig>) -> Node {
        Node {
            id: id.to_string(),
            node_type: node_type.to_string(),
            name: id.to_string(),
            description: String::new(),
            inputs: required_inputs,
            outputs: vec![],
            data_schema: json!({}),
            data: json!({}),
            data_ui_schema: json!({}),
            position: Position { x: 0.0, y: 0.0 },
            component: String::new(),
            executor_id: String::new(),
            status: String::new(),
            extra: None,
        }
    }

    // Helper function to create an edge
    fn create_edge(source_node_id: &str, target_node_id: &str, target_endpoint_id: &str) -> Edge {
        Edge {
            id: format!("{}->{}", source_node_id, target_node_id),
            source: EndpointRef {
                node_id: source_node_id.to_string(),
                endpoint_id: String::new(),
            },
            target: EndpointRef {
                node_id: target_node_id.to_string(),
                endpoint_id: target_endpoint_id.to_string(),
            },
        }
    }

    #[test]
    fn test_find_start_node() {
        let start_node = create_node("start", "start", vec![]);
        let node_b = create_node("b", "task", vec![]);
        let nodes = vec![start_node.clone(), node_b];

        let planner = WorkflowPlanner::new(nodes, vec![]);
        assert_eq!(planner.start_node.id, "start");
    }

    #[test]
    #[should_panic(expected = "Multiple start nodes found. Ensure only one start node is defined.")]
    fn test_multiple_start_nodes_panic() {
        let start_node1 = create_node("start1", "start", vec![]);
        let start_node2 = create_node("start2", "start", vec![]);
        let nodes = vec![start_node1, start_node2];

        // Should panic due to multiple start nodes
        WorkflowPlanner::new(nodes, vec![]);
    }

    #[test]
    fn test_next_nodes_single_path() {
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node("a", "task", vec![]);

        // 设置 target_endpoint_id 参数为空字符串，因为 node_a 没有必需输入
        let edge = create_edge("start", "a", "");

        let mut planner =
            WorkflowPlanner::new(vec![start_node.clone(), node_a.clone()], vec![edge]);

        let next = planner.next_nodes(&start_node, &json!({}));
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].id, "a");
    }

    #[test]
    fn test_next_nodes_with_branching() {
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node("a", "task", vec![]);
        let node_b = create_node("b", "task", vec![]);

        // 设置 target_endpoint_id 参数为空字符串，因为 node_a 和 node_b 没有必需输入
        let edges = vec![create_edge("start", "a", ""), create_edge("start", "b", "")];

        let mut planner = WorkflowPlanner::new(
            vec![start_node.clone(), node_a.clone(), node_b.clone()],
            edges,
        );

        let next = planner.next_nodes(&start_node, &json!({}));
        assert_eq!(next.len(), 2);
        assert!(next.iter().any(|node| node.id == "a"));
        assert!(next.iter().any(|node| node.id == "b"));
    }

    #[test]
    fn test_node_with_required_inputs() {
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node(
            "a",
            "task",
            vec![EndpointConfig {
                id: "input".to_string(), // Ensure the required input has this specific ID
                name: "input".to_string(),
                required: true,
                data_type: "json".to_string(),
                display_type: "input".to_string(),
                description: "Required input".to_string(),
            }],
        );

        // Set the edge's target.endpoint_id to "input" to satisfy node_a's requirement
        let edge = Edge {
            id: "start->a".to_string(),
            source: EndpointRef {
                node_id: "start".to_string(),
                endpoint_id: "".to_string(),
            },
            target: EndpointRef {
                node_id: "a".to_string(),
                endpoint_id: "input".to_string(), // Connects to node_a's required input
            },
        };

        let mut planner =
            WorkflowPlanner::new(vec![start_node.clone(), node_a.clone()], vec![edge]);

        // Execute start node, then check for next nodes
        let next = planner.next_nodes(&start_node, &json!({}));
        assert_eq!(
            next.len(),
            1,
            "Expected node_a to be executable after start node"
        );
        assert_eq!(next[0].id, "a");
    }

    #[test]
    fn test_prevent_revisiting_nodes() {
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node("a", "task", vec![]);

        // 设置 target_endpoint_id 参数为空字符串，因为 node_a 没有必需输入
        let edge = create_edge("start", "a", "");

        let mut planner =
            WorkflowPlanner::new(vec![start_node.clone(), node_a.clone()], vec![edge]);

        // 执行 start_node，首次应返回 node_a
        let _ = planner.next_nodes(&start_node, &json!({}));

        // 将 node_a 标记为已访问
        planner.visited.insert("a".to_string());

        // 再次调用 next_nodes，node_a 不应再返回
        let next = planner.next_nodes(&start_node, &json!({}));
        assert!(
            next.is_empty(),
            "No nodes should be executable since 'a' was already visited."
        );
    }

    #[test]
    fn test_next_nodes_multiple_required_inputs() {
        // Test case where a node has multiple required inputs
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node(
            "a",
            "task",
            vec![
                EndpointConfig {
                    id: "input1".to_string(),
                    name: "input1".to_string(),
                    required: true,
                    data_type: "json".to_string(),
                    display_type: "input".to_string(),
                    description: "Required input 1".to_string(),
                },
                EndpointConfig {
                    id: "input2".to_string(),
                    name: "input2".to_string(),
                    required: true,
                    data_type: "json".to_string(),
                    display_type: "input".to_string(),
                    description: "Required input 2".to_string(),
                },
            ],
        );

        // Create edges for both required inputs
        let edge1 = create_edge("start", "a", "input1");
        let edge2 = create_edge("start", "a", "input2");
        let mut planner =
            WorkflowPlanner::new(vec![start_node.clone(), node_a.clone()], vec![edge1, edge2]);

        // Execute start node, then check for next nodes
        let next = planner.next_nodes(&start_node, &json!({}));
        assert_eq!(
            next.len(),
            1,
            "Expected node_a to be executable after start node"
        );
        assert_eq!(next[0].id, "a");
    }

    #[test]
    fn test_skip_visited_nodes() {
        // Test that a node is not revisited if already executed
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node("a", "task", vec![]);
        let edge = create_edge("start", "a", "");
        let mut planner =
            WorkflowPlanner::new(vec![start_node.clone(), node_a.clone()], vec![edge]);

        // Execute node_a and mark as visited
        planner.visited.insert("a".to_string());
        let next = planner.next_nodes(&start_node, &json!({}));

        assert!(
            next.is_empty(),
            "Expected no next nodes as 'a' was already visited."
        );
    }

    #[test]
    fn test_missing_target_node() {
        // Test case where an edge points to a non-existent node
        let start_node = create_node("start", "start", vec![]);
        let edge = create_edge("start", "nonexistent_node", "");
        let mut planner = WorkflowPlanner::new(vec![start_node.clone()], vec![edge]);

        let next = planner.next_nodes(&start_node, &json!({}));
        assert!(
            next.is_empty(),
            "Expected no next nodes as target node does not exist."
        );
    }

    #[test]
    fn test_circular_dependency_detection() {
        // Test circular dependency scenario where nodes reference each other
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node("a", "task", vec![]);
        let node_b = create_node("b", "task", vec![]);

        // Circular edges: start -> a, a -> b, b -> a
        let edge1 = create_edge("start", "a", "");
        let edge2 = create_edge("a", "b", "");
        let edge3 = create_edge("b", "a", "");

        let mut planner = WorkflowPlanner::new(
            vec![start_node.clone(), node_a.clone(), node_b.clone()],
            vec![edge1, edge2, edge3],
        );

        let next = planner.next_nodes(&start_node, &json!({}));
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].id, "a");

        // Mark node_a as visited, so it should not be revisited in a circular loop
        planner.visited.insert("a".to_string());
        let next = planner.next_nodes(&node_a, &json!({}));
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].id, "b");
    }

    #[test]
    fn test_node_with_optional_inputs() {
        // Test case where a node has both required and optional inputs
        let start_node = create_node("start", "start", vec![]);
        let node_a = create_node(
            "a",
            "task",
            vec![
                EndpointConfig {
                    id: "required_input".to_string(),
                    name: "required_input".to_string(),
                    required: true,
                    data_type: "json".to_string(),
                    display_type: "input".to_string(),
                    description: "Required input".to_string(),
                },
                EndpointConfig {
                    id: "optional_input".to_string(),
                    name: "optional_input".to_string(),
                    required: false,
                    data_type: "json".to_string(),
                    display_type: "input".to_string(),
                    description: "Optional input".to_string(),
                },
            ],
        );

        let required_edge = create_edge("start", "a", "required_input");
        let optional_edge = create_edge("start", "a", "optional_input");
        let mut planner = WorkflowPlanner::new(
            vec![start_node.clone(), node_a.clone()],
            vec![required_edge, optional_edge],
        );

        let next = planner.next_nodes(&start_node, &json!({}));
        assert_eq!(
            next.len(),
            1,
            "Expected node_a to be executable with only required input satisfied"
        );
        assert_eq!(next[0].id, "a");
    }
}
