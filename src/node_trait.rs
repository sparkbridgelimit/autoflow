use arrow::record_batch::RecordBatch;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

/// 通用的工作流节点定义
/// 1. 可以有多个输入和输出端点
/// 2. 输入输出都以arrow数据格式定义
/// 3. 每个实现了NodeTrait的自定义节点都可以通过配置的方式声明有哪些输入、输出端点, 用于前端渲染节点端点
/// 4. 每个实现了NodeTrait的自定义节点都可以通过配置的方式声明data的json schema, 用于校验数据类型
/// 5. 每个实现了NodeTrait的自定义节点都可以通过配置的方式声明有data的ui schema, 用于前端渲染参数面板的表单
/// 6. 每个实现了NodeTrait的自定义节点都要自行实现execute方法
/// 7. 每个实现了NodeTrait的自定义节点的execute方法内可以方便的使用NodeTrait默认提供的xxx方法读取输入端点的参数
/// 8. 每个实现了NodeTrait的自定义节点的execute方法内可以方便的使用NodeTrait默认提供的xxx方法读取输入端点的参数
/// 9. 每个实现了NodeTrait的自定义节点的execute方法内部的每一个输出都是一个独立的arrow数据格式
/// 
pub trait NodeTrait {
    /// 每个节点必须实现的 `execute` 方法，用于执行节点的计算逻辑
    fn execute(&mut self) -> Result<HashMap<String, RecordBatch>, Box<dyn Error>>;

    /// 获取输入端点的 Arrow 数据，输入端点的名称为 key
    fn get_input(&self, key: &str) -> Option<&RecordBatch>;

    /// 获取输出端点的 Arrow 数据，输出端点的名称为 key
    fn get_output(&self, key: &str) -> Option<&RecordBatch>;

    /// 声明每个节点的输入端点和输出端点，提供给前端用于动态渲染节点
    fn endpoints(&self) -> HashMap<String, ArrowType>;

    /// 定义数据的 JSON Schema，用于验证输入数据的结构和类型
    fn data_schema(&self) -> Value;

    /// 定义 UI Schema，用于动态生成前端表单配置面板
    fn ui_schema(&self) -> Value;

    /// 提供一个辅助方法，根据 data_schema 的定义获取参数，自动转为期望的类型
    fn get_data(&self, key: &str) -> Option<Value>;

    /// 默认实现：设置 Arrow 输入数据（方便管理端点）
    fn set_input(&mut self, key: &str, data: RecordBatch);

    /// 默认实现：设置 Arrow 输出数据（方便管理端点）
    fn set_output(&mut self, key: &str, data: RecordBatch);
}

/// 假设一个枚举类型表示不同的 Arrow 数据类型
pub enum ArrowType {
    Int32,
    Float64,
    Utf8,
}


pub trait CustomNodeTrait: NodeTrait {
    // 自定义节点的附加功能
    fn execute_custom_logic(&self);
}