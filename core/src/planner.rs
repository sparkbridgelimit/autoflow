use std::collections::{HashMap, HashSet};

use crate::{edge::Edge, node::Node};

pub struct Planner {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_map: std::collections::HashMap<String, Node>,
    pub edge_map: std::collections::HashMap<String, Vec<Edge>>,
    // 已经运行过的节点
    pub visited: HashSet<String>,
    // 提前预置的开始节点
    pub start_node: Node,
}

impl Planner {
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

        Planner {
            nodes,
            edges,
            node_map,
            edge_map,
            visited: HashSet::new(),
            start_node,
        }
    }

    // 查找节点类型为 "start" 的起始节点
    fn find_start_node(nodes: &[Node]) -> Option<&Node> {
        for node in nodes {
            if node.node_type == "start" {
                return Some(node);
            }
        }
        None
    }

    pub fn next_nodes(&mut self, node: &Node, _data: &serde_json::Value) -> Vec<Node> {
        self.visited.insert(node.id.clone());

        let mut next_nodes = Vec::new();
        let mut seen_nodes = HashSet::new();

        // 找到已执行节点的所有输出边
        let outgoing_edges: Vec<&Edge> = self
            .edges
            .iter()
            .filter(|edge| edge.source.node_id == node.id)
            .collect();

        // 对于每个输出边，处理目标节点
        for edge in outgoing_edges {
            let target_node_id = &edge.target.node_id;

            // 如果目标节点已被访问，跳过
            if self.visited.contains(target_node_id) {
                continue;
            }

            // 避免重复处理同一节点
            if seen_nodes.contains(target_node_id) {
                continue;
            }

            // 获取目标节点
            let target_node = match self.node_map.get(target_node_id) {
                Some(node) => node,
                None => continue, // 节点不存在，跳过
            };

            // 检查目标节点的所有必需输入是否已满足
            let mut ready = true;

            for input_endpoint in &target_node.inputs {
                if !input_endpoint.required {
                    continue;
                }

                let mut input_satisfied = false;

                // 查找指向该输入端点的边，其源节点已被执行
                for edge in self.edges.iter().filter(|e| {
                    e.target.node_id == target_node.id && e.target.endpoint_id == input_endpoint.id
                }) {
                    if self.visited.contains(&edge.source.node_id) {
                        input_satisfied = true;
                        break;
                    }
                }

                // 如果有必需输入未满足，标记为不可执行
                if !input_satisfied {
                    ready = false;
                    break;
                }
            }

            // 如果所有必需输入都已满足，添加到可执行节点列表
            if ready {
                next_nodes.push(target_node.clone());
                seen_nodes.insert(target_node_id.clone());
            }
        }
        next_nodes
    }
}
