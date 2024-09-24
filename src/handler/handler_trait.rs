pub trait NodeHandlerTrait {
  // 每个结构体都必须实现 node_id 方法
  fn node_id(&self) -> &str;

  // 用户需要实现 execute 函数来执行节点自定义逻辑
  fn execute(&self);
}