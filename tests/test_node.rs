#[cfg(test)]
mod tests {
    use autoflow::node::Node;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn test_node_from_json() {
        let json_data = json!({
            "id": "start",
            "node_type": "start",
            "name": "Start Node",
            "description": "This is the start node of the workflow.",
            "implementation": "start_function",
            "inputs": [],
            "outputs": [
                {
                    "id": "output_001",
                    "name": "Start Output",
                    "required": false,
                    "endpoint_type": "arrow",
                    "display_type": "single_value",
                    "description": "The output of the start node"
                }
            ],
            "data_schema": {},
            "data": {},
            "data_ui_schema": {},
            "component": "start_component",
            "execute": "start_logic",
            "status": "PENDING",
            "extra": null
        });

        let node = Node::from_json(&json_data).unwrap();
        assert_eq!(node.id, "start");
        assert_eq!(node.node_type, "start");
        assert_eq!(node.name, "Start Node");
        assert_eq!(node.description, "This is the start node of the workflow.");
        assert_eq!(node.implementation, "start_function");
        assert!(node.inputs.is_empty());
        assert_eq!(node.outputs.len(), 1);
        assert_eq!(node.outputs[0].id, "output_001");
        assert_eq!(node.outputs[0].name, "Start Output");
        assert_eq!(node.outputs[0].required, false);
        assert_eq!(node.outputs[0].endpoint_type, "arrow");
        assert_eq!(node.outputs[0].display_type, "single_value");
        assert_eq!(node.outputs[0].description, "The output of the start node");
        assert_eq!(node.component, "start_component");
        assert_eq!(node.execute, "start_logic");
        assert_eq!(node.status, "PENDING");
        assert!(node.extra.is_none());
    }

    #[test]
    fn test_node_from_json_path() {
        // Use the correct file path relative to the project root
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("src/nodes.json");

        let node = Node::from_json_path(file_path.to_str().unwrap()).unwrap();

        assert_eq!(node.id, "start");
        assert_eq!(node.node_type, "start");
        assert_eq!(node.name, "Start Node");
        assert_eq!(node.description, "This is the start node of the workflow.");
        assert_eq!(node.implementation, "start_function");
        assert!(node.inputs.is_empty());
        assert_eq!(node.outputs.len(), 1);
        assert_eq!(node.outputs[0].id, "output_001");
        assert_eq!(node.outputs[0].name, "Start Output");
        assert_eq!(node.outputs[0].required, false);
        assert_eq!(node.outputs[0].endpoint_type, "arrow");
        assert_eq!(node.outputs[0].display_type, "single_value");
        assert_eq!(node.outputs[0].description, "The output of the start node");
        assert_eq!(node.component, "start_component");
        assert_eq!(node.execute, "start_logic");
        assert_eq!(node.status, "PENDING");
        assert!(node.extra.is_none());
    }
}
