use crate::endpoint::Endpoint;

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: String,                      // Edge 的唯一标识符
    pub source: Endpoint,                // 起点（输出端点）
    pub target: Endpoint,                // 终点（输入端点）
    pub conditions: Vec<String>,         // 条件列表，多个表达式
    pub selectable: bool,                // 是否可选择
}

pub trait EdgeTrait {
    // 定义一个方法用于获取Edge的唯一标识符
    fn get_id(&self) -> &str;

    // 定义一个方法用于获取Edge的源端点
    fn get_source(&self) -> &Endpoint;

    // 定义一个方法用于获取Edge的目标端点
    fn get_target(&self) -> &Endpoint;

    // 定义一个方法用于返回Edge的条件列表
    fn get_conditions(&self) -> &Vec<String>;

    // 定义一个方法用于检查Edge是否可选择
    fn is_selectable(&self) -> bool;

    // 定义一个方法用于检查条件 (仅定义，暂不实现)
    fn check_conditions(&self, data: &std::collections::HashMap<String, String>) -> bool;
}

