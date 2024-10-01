use serde::{Deserialize, Serialize};

use crate::{
    edge::Edge,
    endpoint::EndpointRef,
    node::{Node, Position},
    reactflow::{ ReactFlow, ReactFlowEdge, ReactFlowNode},
    viewport::ViewPort,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Flow {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
    pub viewport: ViewPort,
    pub zoom: i32,
}

pub trait ReactflowTrait {
    fn from(react_flow: ReactFlow) -> Flow;
    fn to(&self) -> ReactFlow;
}

impl ReactflowTrait for Flow {
    fn from(react_flow: ReactFlow) -> Flow {
        let nodes = react_flow
            .nodes
            .into_iter()
            .map(|react_node| {
                // 从 react_node.data 提取出 data, data_schema, data_ui_schema
                let node_data = react_node
                    .data
                    .get("data")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                let data_schema = react_node
                    .data
                    .get("data_schema")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                let data_ui_schema = react_node
                    .data
                    .get("data_ui_schema")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);

                Node {
                    id: react_node.id.clone(),
                    node_type: react_node.type_.clone(),
                    name: "".to_string(),        // 这里可以根据需要添加
                    description: "".to_string(), // 同上
                    inputs: vec![],              // 需要通过 react_node.data 中的值填充
                    outputs: vec![],             // 同上
                    data_schema,                 // 映射到 Node 中的 data_schema
                    data: node_data,             // 映射到 Node 中的 data
                    data_ui_schema,              // 映射到 Node 中的 data_ui_schema
                    component: "".to_string(),   // 根据 react_node.data 填充
                    executor_id: "".to_string(), // 根据 react_node.data 填充
                    status: "".to_string(),      // 根据 react_node.data 填充
                    extra: None,                 // 根据需要填充
                    position: react_node.position,
                }
            })
            .collect();

        let edges = react_flow
            .edges
            .into_iter()
            .map(|react_edge| Edge {
                id: react_edge.id.clone(),
                source: EndpointRef {
                    node_id: react_edge.source.clone(),
                    endpoint_id: react_edge.source_handle.clone(),
                },
                target: EndpointRef {
                    node_id: react_edge.target.clone(),
                    endpoint_id: react_edge.target_handle.clone(),
                },
            })
            .collect();

        Flow {
            nodes,
            edges,
            viewport: react_flow.viewport,
            zoom: react_flow.zoom,
        }
    }

    fn to(&self) -> ReactFlow {
        let nodes = self
            .nodes
            .iter()
            .map(|node| {
                // 将 Node 的 data, data_schema, data_ui_schema 打包到 react_node 的 data 字段中
                let mut reactflow_data = serde_json::Map::new();
                reactflow_data.insert("data".to_string(), node.data.clone());
                reactflow_data.insert("data_schema".to_string(), node.data_schema.clone());
                reactflow_data.insert("data_ui_schema".to_string(), node.data_ui_schema.clone());

                ReactFlowNode {
                    id: node.id.clone(),
                    type_: node.node_type.clone(),
                    position: Position { x: 0.0, y: 0.0 }, // 这里可以根据需要调整位置
                    data: serde_json::Value::Object(reactflow_data), // 打包成 Object 放入 data 字段
                }
            })
            .collect();

        let edges = self
            .edges
            .iter()
            .map(|edge| ReactFlowEdge {
                id: edge.id.clone(),
                source: edge.source.node_id.clone(),
                source_handle: edge.source.endpoint_id.clone(),
                target: edge.target.node_id.clone(),
                target_handle: edge.target.endpoint_id.clone(),
            })
            .collect();

        ReactFlow {
            nodes,
            edges,
            viewport: self.viewport.clone(),
            zoom: self.zoom,
        }
    }
}
