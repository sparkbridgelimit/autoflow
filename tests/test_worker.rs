#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use autoflow::{
        fetcher::LocalQueueFetcher,
        handler2::TaskHandler,
        task::{Task, TaskStatus},
        worker::Worker,
    };
    use serde_json::json;

    use crate::tests;

    struct MockTaskHandler;

    #[async_trait::async_trait]
    impl TaskHandler for MockTaskHandler {
        async fn before(&self) {
            println!("Mock: before task");
        }

        async fn handle(&self) {
            println!("Mock: handling task");
        }

        async fn after(&self) {
            println!("Mock: after task");
        }

        fn for_task(&self) -> &'static str {
            "mock"
        }
    }

    #[tokio::test] // 使用 tokio::test 来启用异步测试
    async fn test_worker_processes_tasks_correctly() {
        // 创建任务队列和 Worker
        let fetcher = Arc::new(LocalQueueFetcher);
        let mut worker = Worker::new(fetcher);
        worker.with_limit(1);

        // 添加任务处理器
        worker.add_handler("mock".to_string(), MockTaskHandler);

        // 模拟任务放入队列
        {
            let mut queue = worker.task_queue.lock().await;
            queue.push(Task {
                task_type: "mock".to_string(),
                data: json!({}),
                status: tests::TaskStatus::Queued,
            });
        }

        // 用 tokio::time::timeout 设置超时时间
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), worker.run()).await;

        assert!(result.is_ok(), "The worker run timed out");
    }

    #[tokio::test] // 使用 tokio::test 来启用异步测试
    async fn test_worker_handles_unknown_task_type() {
        // 创建任务队列和 Worker，模拟任务处理
        let fetcher = Arc::new(LocalQueueFetcher); // 假设 LocalQueueFetcher 实现了 Fetcher
        let mut worker = Worker::new(fetcher);
        worker.with_limit(10);

        // 添加 mock 处理器
        worker.add_handler("mock".to_string(), MockTaskHandler);

        // 模拟任务放入队列
        {
            let mut queue = worker.task_queue.lock().await;
            queue.push(Task {
                task_type: "mock".to_string(),
                data: json!({}),
                status: TaskStatus::Queued,
            });

            // 添加一个未知任务类型
            queue.push(Task {
                task_type: "unknown".to_string(), // 未注册的任务类型
                data: json!({}),
                status: TaskStatus::Queued,
            });

            // 添加另一个已知的任务
            queue.push(Task {
                task_type: "mock".to_string(),
                data: json!({}),
                status: TaskStatus::Queued,
            });
        }

        // 用 tokio::time::timeout 设置超时时间
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), worker.run()).await;

        assert!(result.is_ok(), "The worker run timed out");
    }
}
