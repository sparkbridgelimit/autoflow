use std::result::Result::Ok;
use std::{fmt, time::Instant};

use serde::{Deserialize, Serialize};

// 任务状态的枚举类型
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Success,
    Failed,
}

pub trait TaskHandler<T> {
    fn before(&mut self, data: &T) -> Result<(), String>;
    fn handle(&mut self, data: &T) -> Result<(), String>;
    fn after(&mut self, data: &T) -> Result<(), String>;
}

pub struct Task<T> {
    // 当前任务要处理的数据
    pub data: T,

    // 当前任务的handler函数
    pub handler: Box<dyn TaskHandler<T>>,

    // 当前任务状态
    pub status: TaskStatus,

    // 最大重试次数
    pub max_attempts: i32,

    // 当前的重试次数
    pub attempts: i32,

    // 执行开始时间，用于控制超时, 统计耗时
    pub start_time: Option<Instant>,

    // 结束时间, 统计耗时
    pub end_time: Option<Instant>,
}

impl<T> Task<T> {
    // 新建任务的构造函数
    pub fn new(data: T, handler: Box<dyn TaskHandler<T>>, max_attempts: i32) -> Self {
        Task {
            data,
            handler,
            max_attempts,
            status: TaskStatus::Pending,
            attempts: 0,
            start_time: None,
            end_time: None,
        }
    }

    // 更新任务状态
    fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    // 打印任务的执行时间统计
    fn print_duration(&self) {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            let duration = end.duration_since(start);
            println!("Task completed in {:?}", duration);
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task {{ data: {:?}, status: {:?}, attempts: {}, start_time: {:?}, end_time: {:?} }}",
            self.data, self.status, self.attempts, self.start_time, self.end_time
        )
    }
}

pub trait TaskExecutorTrait<T> {
    // 执行一个任务块并管理后续的操作或错误处理
    fn execute(&mut self) -> Result<(), String>;

    // 获取任务的 handler
    fn get_handler(&self) -> &dyn TaskHandler<T>;

    // 将任务结果添加到kv缓存上
    fn add_result(&self);
}

impl<T> TaskExecutorTrait<T> for Task<T> {
    fn execute(&mut self) -> Result<(), String> {
        self.start_time = Some(Instant::now());
        self.update_status(TaskStatus::Running);

        println!("Task execution started...");

        let result = loop {
            // 如果达到最大重试次数，则失败
            if self.attempts >= self.max_attempts {
                self.update_status(TaskStatus::Failed);
                break Err("Maximum retry attempts reached".to_string());
            }

            let data = &self.data;

            let before_result = self.handler.before(data);
            let handle_result = self.handler.handle(data);
            let after_result = self.handler.after(data);

            match (before_result, handle_result, after_result) {
                (Ok(_), Ok(_), Ok(_)) => {
                    self.end_time = Some(Instant::now());
                    self.update_status(TaskStatus::Success);
                    break Ok(());
                }
                (before_res, handle_res, after_res) => {
                    self.attempts += 1; // 增加重试次数
                    println!(
                      "Task execution failed. Before: {:?}, Handle: {:?}, After: {:?}. Retrying {}/{}",
                      before_res, handle_res, after_res, self.attempts, self.max_attempts
                  );
                }
            }
        };

        // 如果任务成功或失败，则记录任务结果
        match &result {
            Ok(_) => {
                self.add_result(); // 添加任务结果
                println!("Task executed successfully.");
            }
            Err(e) => {
                println!("Task execution failed with error: {}", e);
            }
        }

        // 打印任务的耗时统计
        self.print_duration();

        result
    }

    // 获取当前任务的 handler
    fn get_handler(&self) -> &dyn TaskHandler<T> {
        &*self.handler
    }

    // 将任务结果添加到缓存或数据库
    fn add_result(&self) {
        // 模拟保存任务的执行结果
        println!("Task result has been stored.");
    }
}
