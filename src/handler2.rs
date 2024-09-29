pub trait TaskHandler: Send + Sync {
    fn for_task (&self) -> &'static str;
    
    /// 在处理任务之前调用的钩子
    fn before(&self) {
        // 默认实现为空，可以被覆盖
    }

    /// 处理任务的主要逻辑，必须被实现
    fn handle(&self);

    /// 在处理任务之后调用的钩子
    fn after(&self) {
        // 默认实现为空，可以被覆盖
    }
}

/// 扩展 Trait，用于提供 exec 方法
pub trait TaskHandlerExec {
    /// 执行任务处理流程：before -> handle -> after
    fn exec(&self);
}

/// 为所有实现了 TaskHandler 的类型提供 exec 方法
impl<T: TaskHandler> TaskHandlerExec for T {
    fn exec(&self) {
        self.before();
        self.handle();
        self.after();
    }
}
