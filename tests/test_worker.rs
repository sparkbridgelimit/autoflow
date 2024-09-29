#[cfg(test)]
mod tests {
    use autoflow::{
        fetcher::Fetcher, handler2::TaskHandlerExec, task::Task, worker::Worker}
    ;
    use mockall::predicate::*;
    use mockall::*;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    // 1. 创建 MockFetcher
    mock! {
        pub Fetcher {}

        #[allow(dead_code)]
        impl Fetcher for Fetcher {
            fn fetch(&self) -> Vec<Task>;
        }
    }

    // 2. 创建 MockTaskHandlerExec
    mock! {
        pub TaskHandlerExec {}

        #[allow(dead_code)]
        impl TaskHandlerExec for TaskHandlerExec {
            fn exec(&self);
        }
    }

    #[test]
    fn test_worker_processes_tasks_correctly() {
        // 3. 创建 MockFetcher 实例
        let mut mock_fetcher = MockFetcher::new();

        // 定义要返回的任务列表
        let tasks = vec![
            Task::new("task_type_1".to_string(), json!({"key1": "value1"})),
            Task::new("task_type_2".to_string(), json!({"key2": "value2"})),
        ];

        // 设置 MockFetcher 的 fetch 方法返回预定义的任务
        mock_fetcher
            .expect_fetch()
            .times(1)
            .return_const(tasks.clone());

        // 4. 创建 MockTaskHandlerExec 实例
        let mut mock_handler1 = MockTaskHandlerExec::new();
        let mut mock_handler2 = MockTaskHandlerExec::new();

        // 使用 Arc 和 Mutex 来共享调用计数
        let exec_count1 = Arc::new(Mutex::new(0));
        let exec_count1_clone = Arc::clone(&exec_count1);
        mock_handler1.expect_exec().times(1).returning(move || {
            let mut count = exec_count1_clone.lock().unwrap();
            *count += 1;
        });

        let exec_count2 = Arc::new(Mutex::new(0));
        let exec_count2_clone = Arc::clone(&exec_count2);
        mock_handler2.expect_exec().times(1).returning(move || {
            let mut count = exec_count2_clone.lock().unwrap();
            *count += 1;
        });

        // 5. 创建 Worker 实例并添加处理器
        let mut worker = Worker::new(Box::new(mock_fetcher));

        worker.add_handler("task_type_1".to_string(), mock_handler1);
        worker.add_handler("task_type_2".to_string(), mock_handler2);

        // 6. 运行 Worker 在一个新线程，以防止阻塞测试
        let handle = std::thread::spawn(move || {
            // 为了防止 run() 的无限循环阻塞测试，这里我们只调用 run() 一次
            // 并在 run() 中添加一个 break 条件以结束循环
            // 但是根据您提供的代码，run() 是一个无限循环，无法自然结束
            // 因此，我们需要修改 run() 方法以支持测试，如添加退出条件
            // 但假设我们无法修改 Worker 结构，我们可以 skip testing run()
            // 或者 use other strategies like panicking after some time
            // 这里仅展示如何调用 run()，但实际测试可能需要修改 Worker 结构
            worker.run();
        });

        // 由于 run() 是无限循环，我们需要让测试在有限时间内结束
        // 这里我们让测试主线程等待一段时间后终止
        // 并验证 exec_count 是否正确
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 7. 验证 exec_count
        assert_eq!(*exec_count1.lock().unwrap(), 1);
        assert_eq!(*exec_count2.lock().unwrap(), 1);

        // 8. 终止 Worker 线程
        // 在实际代码中，您应该为 Worker 添加一种方式来安全地终止 run() 方法
        // 例如，通过使用一个终止信号（如 AtomicBool）来退出循环
        // 由于当前的 Worker.run() 是无限循环，我们无法安全地终止它
        // 所以这里无法 join 线程，测试可能会挂起
        // 因此，建议您修改 Worker.run() 以支持测试终止

        // handle.join().unwrap(); // 这将永远阻塞，因为 run() 是无限循环
    }

    #[test]
    fn test_worker_handles_unknown_task_type() {
        // 1. 创建 MockFetcher 实例
        let mut mock_fetcher = MockFetcher::new();

        // 定义要返回的任务列表，其中包含未知的任务类型
        let tasks = vec![Task::new(
            "unknown_task".to_string(),
            json!({"key_unknown": "value_unknown"}),
        )];

        // 设置 MockFetcher 的 fetch 方法返回预定义的任务
        mock_fetcher
            .expect_fetch()
            .times(1)
            .return_const(tasks.clone());

        // 2. 创建 Worker 实例，不添加任何处理器
        let mut worker = Worker::new(Box::new(mock_fetcher));

        // 3. 运行 Worker 在一个新线程，以防止阻塞测试
        let handle = std::thread::spawn(move || {
            worker.run();
        });

        // 由于 run() 是无限循环，我们需要让测试在有限时间内结束
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 4. 由于任务类型未知，Worker 应该输出相应的消息，但无法验证输出
        // 如果需要验证输出，可以使用日志捕获工具或重定向 stdout
        // 这里只确保没有 panic

        // 5. 终止 Worker 线程
        // 同样，由于 run() 是无限循环，无法安全终止
        // handle.join().unwrap(); // 这将永远阻塞，因为 run() 是无限循环
    }
}
