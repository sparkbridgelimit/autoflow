#[cfg(test)]
mod tests {
    use autoflow::task_executor::{Task, TaskExecutorTrait, TaskHandler, TaskStatus};

    #[test]
    fn test_task_success_execution() {
        struct MockTaskHandler;

        impl<T> TaskHandler<T> for MockTaskHandler {
            fn before(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }

            fn handle(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }

            fn after(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }
        }

        let data = "test_data"; // 可以替换为任何类型的测试数据
        let handler: Box<MockTaskHandler> = Box::new(MockTaskHandler);
        let mut task = Task::new(data, handler, 3);

        let result = task.execute();
        // 检查任务是否成功执行
        assert!(result.is_ok());
        assert_eq!(task.status, TaskStatus::Success);
        assert_eq!(task.attempts, 0);
    }

    #[test]
    fn test_task_retry_on_failure() {
        // 测试任务失败后重试的情况
        struct MockFailTaskHandler {
            attempts_before_success: i32,
            current_attempt: i32,
        }

        impl MockFailTaskHandler {
            fn new(attempts_before_success: i32) -> Self {
                MockFailTaskHandler {
                    attempts_before_success,
                    current_attempt: 0,
                }
            }
        }

        impl<T> TaskHandler<T> for MockFailTaskHandler {
            fn before(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }

            fn handle(&mut self, _data: &T) -> Result<(), String> {
                self.current_attempt += 1;
                if self.current_attempt <= self.attempts_before_success {
                    Err("Failure".to_string())
                } else {
                    Ok(())
                }
            }

            fn after(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }
        }

        let data = "test_data"; // 可以替换为任何类型的测试数据
        let handler = Box::new(MockFailTaskHandler::new(2));
        let mut task: Task<&str> = Task::new(data, handler, 3);
        let result = task.execute();

        // 检查任务是否成功执行
        assert!(result.is_ok()); // 任务应该最终成功
        assert_eq!(task.status, TaskStatus::Success); // 状态应该是 Success
        assert_eq!(task.attempts, 2); // 任务应该重试了2次
    }

    #[test]
    fn test_task_fail_with_zero_max_retries() {
        struct MockFailTaskHandler {
            attempts_before_success: i32,
            current_attempt: i32,
        }

        impl MockFailTaskHandler {
            fn new(attempts_before_success: i32) -> Self {
                MockFailTaskHandler {
                    attempts_before_success,
                    current_attempt: 0,
                }
            }
        }

        impl<T> TaskHandler<T> for MockFailTaskHandler {
            fn before(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }

            fn handle(&mut self, _data: &T) -> Result<(), String> {
                self.current_attempt += 1;
                if self.current_attempt < self.attempts_before_success {
                    Err("Failure".to_string()) // 模拟失败
                } else {
                    Ok(()) // 模拟成功
                }
            }

            fn after(&mut self, _data: &T) -> Result<(), String> {
                Ok(()) // 模拟成功
            }
        }

        let data = "test_data"; // 可以替换为任何类型的测试数据
        let handler = Box::new(MockFailTaskHandler::new(1)); // 需要1次重试才能成功
        let mut task: Task<&str> = Task::new(data, handler, 0); // 最大重试次数为 0
        let result = task.execute();

        // 检查任务是否失败执行
        assert!(result.is_err()); // 任务应该失败
        assert_eq!(task.status, TaskStatus::Failed); // 状态应该是 Failed
        assert_eq!(task.attempts, 0); // 不应有任何重试
    }
}
