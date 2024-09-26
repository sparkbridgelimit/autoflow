#[cfg(test)]
mod tests {
    use autoflow::{
        edge::Edge,
        endpoint::EndpointRef,
        node::{EndpointConfig, Node},
        planner::Planner,
    };
    use serde_json::json;

    #[test]
    fn test_linear_graph() {
        // 创建节点 A
        let node_a = Node {
            id: "node_a".to_string(),
            node_type: "start".to_string(),
            name: "A".to_string(),
            description: "Start node".to_string(),
            inputs: vec![], // 起始节点没有输入
            outputs: vec![EndpointConfig {
                id: "output_a".to_string(),
                name: "output".to_string(),
                required: true,
                data_type: "arrow".to_string(),
                display_type: "table".to_string(),
                description: "Output endpoint for node A".to_string(),
            }],
            data_schema: serde_json::json!({}),
            data: serde_json::json!({}),
            data_ui_schema: serde_json::json!({}),
            component: "start_component".to_string(),
            executor_id: "executor1".to_string(),
            status: "Pending".to_string(),
            extra: None,
        };

        // 创建节点 B
        let node_b = Node {
            id: "node_b".to_string(),
            node_type: "normal".to_string(),
            name: "B".to_string(),
            description: "Middle node".to_string(),
            inputs: vec![EndpointConfig {
                id: "input_b".to_string(),
                name: "input".to_string(),
                required: true,
                data_type: "json".to_string(),
                display_type: "text".to_string(),
                description: "Input endpoint for node B".to_string(),
            }],
            outputs: vec![EndpointConfig {
                id: "output_b".to_string(),
                name: "output".to_string(),
                required: true,
                data_type: "arrow".to_string(),
                display_type: "table".to_string(),
                description: "Output endpoint for node B".to_string(),
            }],
            data_schema: serde_json::json!({}),
            data: serde_json::json!({}),
            data_ui_schema: serde_json::json!({}),
            component: "normal_component".to_string(),
            executor_id: "executor2".to_string(),
            status: "Pending".to_string(),
            extra: None,
        };

        // 创建节点 C
        let node_c = Node {
            id: "node_c".to_string(),
            node_type: "end".to_string(),
            name: "C".to_string(),
            description: "End node".to_string(),
            inputs: vec![EndpointConfig {
                id: "input_c".to_string(),
                name: "input".to_string(),
                required: true,
                data_type: "json".to_string(),
                display_type: "text".to_string(),
                description: "Input endpoint for node C".to_string(),
            }],
            outputs: vec![], // 结束节点没有输出
            data_schema: serde_json::json!({}),
            data: serde_json::json!({}),
            data_ui_schema: serde_json::json!({}),
            component: "end_component".to_string(),
            executor_id: "executor3".to_string(),
            status: "Pending".to_string(),
            extra: None,
        };

        // 创建边 edge1 连接 node_a -> node_b
        let edge1 = Edge {
            id: "edge1".to_string(),
            source: EndpointRef {
                node_id: "node_a".to_string(),
                endpoint_id: "output_a".to_string(),
            },
            target: EndpointRef {
                node_id: "node_b".to_string(),
                endpoint_id: "input_b".to_string(),
            },
        };

        // 创建边 edge2 连接 node_b -> node_c
        let edge2 = Edge {
            id: "edge2".to_string(),
            source: EndpointRef {
                node_id: "node_b".to_string(),
                endpoint_id: "output_b".to_string(),
            },
            target: EndpointRef {
                node_id: "node_c".to_string(),
                endpoint_id: "input_c".to_string(),
            },
        };

        let mut planner: Planner = Planner::new(
            vec![node_a.clone(), node_b.clone(), node_c.clone()],
            vec![edge1, edge2],
        );

        let start_node = planner.start_node.clone();
        let next = planner.next_nodes(&start_node, &serde_json::Value::Null);

        // Assertions to verify the correctness
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].id, "node_b");
    }

    #[test]
    fn test_branch_graph() {
        // Test a graph where the start node branches to multiple nodes

        // Create start node A
        let node_a = Node {
            id: "node_a".to_string(),
            node_type: "start".to_string(),
            name: "A".to_string(),
            description: "Start node".to_string(),
            inputs: vec![],
            outputs: vec![EndpointConfig {
                id: "output_a".to_string(),
                name: "output".to_string(),
                required: true,
                data_type: "json".to_string(),
                display_type: "text".to_string(),
                description: "Output endpoint for node A".to_string(),
            }],
            data_schema: json!({}),
            data: json!({}),
            data_ui_schema: json!({}),
            component: "start_component".to_string(),
            executor_id: "executor1".to_string(),
            status: "Pending".to_string(),
            extra: None,
        };

        // Create node B
        let node_b = Node {
            id: "node_b".to_string(),
            node_type: "normal".to_string(),
            name: "B".to_string(),
            description: "Branch node B".to_string(),
            inputs: vec![EndpointConfig {
                id: "input_b".to_string(),
                name: "input".to_string(),
                required: true,
                data_type: "json".to_string(),
                display_type: "text".to_string(),
                description: "Input endpoint for node B".to_string(),
            }],
            outputs: vec![],
            data_schema: json!({}),
            data: json!({}),
            data_ui_schema: json!({}),
            component: "normal_component".to_string(),
            executor_id: "executor2".to_string(),
            status: "Pending".to_string(),
            extra: None,
        };

        // Create node C
        let node_c = Node {
            id: "node_c".to_string(),
            node_type: "normal".to_string(),
            name: "C".to_string(),
            description: "Branch node C".to_string(),
            inputs: vec![EndpointConfig {
                id: "input_c".to_string(),
                name: "input".to_string(),
                required: true,
                data_type: "json".to_string(),
                display_type: "text".to_string(),
                description: "Input endpoint for node C".to_string(),
            }],
            outputs: vec![],
            data_schema: json!({}),
            data: json!({}),
            data_ui_schema: json!({}),
            component: "normal_component".to_string(),
            executor_id: "executor3".to_string(),
            status: "Pending".to_string(),
            extra: None,
        };

        // Create edges from node A to node B and node C
        let edge_ab = Edge {
            id: "edge_ab".to_string(),
            source: EndpointRef {
                node_id: "node_a".to_string(),
                endpoint_id: "output_a".to_string(),
            },
            target: EndpointRef {
                node_id: "node_b".to_string(),
                endpoint_id: "input_b".to_string(),
            },
        };

        let edge_ac = Edge {
            id: "edge_ac".to_string(),
            source: EndpointRef {
                node_id: "node_a".to_string(),
                endpoint_id: "output_a".to_string(),
            },
            target: EndpointRef {
                node_id: "node_c".to_string(),
                endpoint_id: "input_c".to_string(),
            },
        };

        // Initialize the planner with nodes and edges
        let mut planner = Planner::new(
            vec![node_a.clone(), node_b.clone(), node_c.clone()],
            vec![edge_ab, edge_ac],
        );

        // Start from the start node
        let start_node = planner.start_node.clone();
        let next_nodes = planner.next_nodes(&start_node, &json!(null));

        // Verify that both node B and node C are ready to execute
        assert_eq!(next_nodes.len(), 2);
        let next_ids: Vec<String> = next_nodes.iter().map(|n| n.id.clone()).collect();
        assert!(next_ids.contains(&"node_b".to_string()));
        assert!(next_ids.contains(&"node_c".to_string()));
    }
}
