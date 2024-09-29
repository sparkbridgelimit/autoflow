use async_trait::async_trait;

#[async_trait]
pub trait TaskHandler: Send + Sync {
    fn for_task(&self) -> &'static str;

    /// 在处理任务之前调用的钩子
    async fn before(&self) {
        // 默认实现为空，可以被覆盖
    }

    /// 处理任务的主要逻辑，必须被实现
    async fn handle(&self);

    /// 在处理任务之后调用的钩子
    async fn after(&self) {
        // 默认实现为空，可以被覆盖
    }
}

/// 扩展 Trait，用于提供 exec 方法
#[async_trait]
pub trait TaskHandlerExec {
    /// 执行任务处理流程：before -> handle -> after
    async fn exec(&self);
}

/// 为所有实现了 TaskHandler 的类型提供 exec 方法
#[async_trait]
impl<T: TaskHandler + Sync> TaskHandlerExec for T {
    async fn exec(&self) {
        self.before().await; // 确保异步方法在此被执行
        self.handle().await; // 同样对 handle 使用 await
        self.after().await; // 确保 after 执行
    }
}
