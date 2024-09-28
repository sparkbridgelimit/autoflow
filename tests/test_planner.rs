#[cfg(test)]
mod tests {
    use autoflow::{
        edge::{Edge, EdgeBuilderTrait},
        endpoint::EndpointRef,
        node::{EndpointConfig, Node, NodeAttrTrait, NodeBuilderTrait},
        planner::Planner,
    };

    // 测试用例 1：线性图
    #[test]
    fn test_linear_graph_optimized() {
        // 创建节点
        let mut node_a = Node::start("A");
        node_a.add_output_endpoint();

        let mut node_b = Node::normal("B");
        node_b.add_input_endpoint();
        node_b.add_output_endpoint();

        let mut node_c = Node::normal("C");
        node_c.add_input_endpoint();

        // 创建边
        let edge1 = Edge::connect(
            &node_a.get_output_ref(0).unwrap(),
            &node_b.get_input_ref(0).unwrap(),
        );
        let edge2 = Edge::connect(
            &node_b.get_output_ref(0).unwrap(),
            &node_c.get_input_ref(0).unwrap(),
        );

        // 初始化 Planner
        let mut planner = Planner::new(
            vec![node_a.clone(), node_b.clone(), node_c.clone()],
            vec![edge1, edge2],
        );

        // 获取起始节点
        let start_node = planner.start_node.clone();

        // 执行起始节点并获取下一个节点
        let next_nodes = planner.next_nodes(&start_node, &serde_json::Value::Null);

        // 验证结果
        assert_eq!(next_nodes.len(), 1);
        assert_eq!(next_nodes[0].name, "B");

        // 执行节点 B
        let node_b_executed = next_nodes[0].clone();
        let next_nodes = planner.next_nodes(&node_b_executed, &serde_json::Value::Null);

        // 验证结果
        assert_eq!(next_nodes.len(), 1);
        assert_eq!(next_nodes[0].name, "C");

        // 执行节点 C
        let node_c_executed = next_nodes[0].clone();
        let next_nodes = planner.next_nodes(&node_c_executed, &serde_json::Value::Null);

        // 应该没有更多节点
        assert_eq!(next_nodes.len(), 0);
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
        let e2 = Edge::connect(&s.get_output_ref(0).unwrap(), &n2.get_input_ref(0).unwrap());

        let mut planner: Planner = Planner::new(vec![s, n1, n2], vec![e1, e2]);

        let start_node = planner.start_node.clone();
        let nodes = planner.next_nodes(&start_node, &serde_json::Value::Null);

        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].name, "process node1");
        assert_eq!(nodes[1].name, "process node2");
    }

    #[test]
    fn test_converging_graph_optimized() {
        // 创建节点
        let mut node_a = Node::start("Node A");
        node_a.add_output_endpoint();

        let mut node_b = Node::normal("Node B");
        node_b.add_output_endpoint();

        let mut node_d = Node::normal("Node D");
        node_d.add_input_endpoint(); // 输入 0
        node_d.add_input_endpoint(); // 输入 1

        // 创建边
        let edge_ad = Edge::connect(
            &node_a.get_output_ref(0).unwrap(),
            &node_d.get_input_ref(0).unwrap(),
        );
        let edge_bd = Edge::connect(
            &node_b.get_output_ref(0).unwrap(),
            &node_d.get_input_ref(1).unwrap(),
        );

        // 初始化 Planner
        let mut planner = Planner::new(
            vec![node_a.clone(), node_b.clone(), node_d.clone()],
            vec![edge_ad, edge_bd],
        );

        // 获取起始节点
        let start_node = planner.start_node.clone();

        // 执行起始节点并获取下一个节点
        let next_nodes = planner.next_nodes(&start_node, &serde_json::Value::Null);

        // 此时，Node D 不应该准备好，因为 Node B 还未执行
        assert_eq!(next_nodes.len(), 1);
        assert_eq!(next_nodes[0].name, "Node B");

        // 执行 Node B
        let node_b_executed = next_nodes[0].clone();
        let next_nodes = planner.next_nodes(&node_b_executed, &serde_json::Value::Null);

        // 现在 Node D 应该准备好执行
        assert_eq!(next_nodes.len(), 1);
        assert_eq!(next_nodes[0].name, "Node D");
    }

    #[test]
    fn test_node_with_optional_input_optimized() {
        // 创建节点
        let mut node_a = Node::start("Node A");
        node_a.add_output_endpoint();

        let mut node_e = Node::normal("Node E");
        node_e.add_input_endpoint(); // 必需输入
        node_e.inputs[0].required = true;
        node_e.add_input_endpoint(); // 可选输入
        node_e.inputs[1].required = false;

        // 创建边
        let edge_ae = Edge::connect(
            &node_a.get_output_ref(0).unwrap(),
            &node_e.get_input_ref(0).unwrap(),
        );

        // 初始化 Planner
        let mut planner = Planner::new(vec![node_a.clone(), node_e.clone()], vec![edge_ae]);

        // 获取起始节点
        let start_node = planner.start_node.clone();

        // 执行起始节点并获取下一个节点
        let next_nodes = planner.next_nodes(&start_node, &serde_json::Value::Null);

        // Node E 应该准备好执行
        assert_eq!(next_nodes.len(), 1);
        assert_eq!(next_nodes[0].name, "Node E");
    }

    #[test]
    fn test_cyclic_graph_optimized() {
        // 创建节点
        let mut node_f = Node::start("Node F");
        node_f.add_input_endpoint(); // 可选输入
        node_f.inputs[0].required = false;
        node_f.add_output_endpoint();

        let mut node_g = Node::normal("Node G");
        node_g.add_input_endpoint();
        node_g.add_output_endpoint();

        // 创建边
        let edge_fg = Edge::connect(
            &node_f.get_output_ref(0).unwrap(),
            &node_g.get_input_ref(0).unwrap(),
        );
        let edge_gf = Edge::connect(
            &node_g.get_output_ref(0).unwrap(),
            &node_f.get_input_ref(0).unwrap(),
        );

        // 初始化 Planner
        let mut planner =
            Planner::new(vec![node_f.clone(), node_g.clone()], vec![edge_fg, edge_gf]);

        // 获取起始节点
        let start_node = planner.start_node.clone();

        // 执行起始节点并获取下一个节点
        let next_nodes = planner.next_nodes(&start_node, &serde_json::Value::Null);

        // Node G 应该准备好执行
        assert_eq!(next_nodes.len(), 1);
        assert_eq!(next_nodes[0].name, "Node G");

        // 执行 Node G 并获取下一个节点
        let node_g_executed = next_nodes[0].clone();
        let next_nodes = planner.next_nodes(&node_g_executed, &serde_json::Value::Null);

        // Node F 已被访问，不应再次执行
        assert_eq!(next_nodes.len(), 0);
    }
}
