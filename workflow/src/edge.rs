use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::{
    endpoint::EndpointRef,
    node::{EndpointConfig, Node},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Edge {
    pub id: String,          // Edge 的唯一标识符
    pub source: EndpointRef, // 起点（输出端点）
    pub target: EndpointRef, // 终点（输入端点）
}

impl Edge {
    pub fn new(
        source_node: &Node,
        target_node: &Node,
        source_endpoint: &EndpointConfig,
        target_endpoint: &EndpointConfig,
    ) -> Self {
        Edge {
            id: nanoid!(8),
            source: EndpointRef {
                node_id: source_node.id.clone(),
                endpoint_id: source_endpoint.id.clone(),
            },
            target: EndpointRef {
                node_id: target_node.id.clone(),
                endpoint_id: target_endpoint.id.clone(),
            },
        }
    }
}

pub trait EdgeBuilderTrait {
    fn connect(source: &EndpointRef, target: &EndpointRef) -> Self;
}

// 为 Edge 实现 EdgeBuilderTrait
impl EdgeBuilderTrait for Edge {
    fn connect(source: &EndpointRef, target: &EndpointRef) -> Self {
        Edge {
            id: nanoid::nanoid!(8), // 生成唯一 ID
            source: source.clone(), // 克隆引用传递的源端点
            target: target.clone(), // 克隆引用传递的目标端点
        }
    }
}

pub trait EdgeTrait {
    // 定义一个方法用于获取Edge的唯一标识符
    fn get_id(&self) -> &str;

    // 定义一个方法用于获取Edge的源端点
    fn get_source(&self) -> &EndpointRef;

    // 定义一个方法用于获取Edge的目标端点
    fn get_target(&self) -> &EndpointRef;

    // 定义一个方法用于检查条件 (仅定义，暂不实现)
    fn check_conditions(&self, data: &std::collections::HashMap<String, String>) -> bool;
}
