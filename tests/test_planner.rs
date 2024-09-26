#[cfg(test)]
mod tests {
    use autoflow::{
        edge::{Edge, EdgeBuilderTrait},
        endpoint::EndpointRef,
        node::{EndpointConfig, Node, NodeAttrTrait, NodeBuilderTrait},
        planner::Planner,
    };

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

    // 分支图
    #[test]
    fn test_branch_graph() {
        let mut s = Node::start("start node");
        s.add_output_endpoint();

        let mut n1 = Node::normal("process node1");
        n1.add_input_endpoint();
        n1.add_output_endpoint();

        let mut n2 = Node::normal("process node2");
        n2.add_input_endpoint();

        let e1 = Edge::connect(&s.get_output_ref(0).unwrap(), &n1.get_input_ref(0).unwrap());
        let e2 = Edge::connect(
            &s.get_output_ref(0).unwrap(),
            &n2.get_input_ref(0).unwrap(),
        );

        let mut planner: Planner = Planner::new(vec![s, n1, n2], vec![e1, e2]);

        let start_node = planner.start_node.clone();
        let nodes = planner.next_nodes(&start_node, &serde_json::Value::Null);

        // Assertions to verify the correctness
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].name, "process node1");
        assert_eq!(nodes[1].name, "process node2");
    }

    // 循环图（A -> B -> C -> A）
}
