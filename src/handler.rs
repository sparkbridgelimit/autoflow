use std::result::Result::Ok;
use std::time::Instant;

pub struct TaskHandler <I, R> {
    pub data: serde_json::Value,

    // 最大重试次数
    pub max_attempts: i32,

    // 当前的重试次数
    pub attempts: i32,

    // 执行开始时间，用于控制超时, 统计耗时
    pub start_time: Option<Instant>,

    // 结束时间, 统计耗时
    pub end_time: Option<Instant>,

    // 可选的前置函数
    pub before: Option<Box<dyn Fn(&I) -> Result<(), String>>>,

    // 核心处理函数，返回类型为 R
    pub handle: Box<dyn Fn(&I) -> Result<R, String>>,

    // 可选的后置函数
    pub after: Option<Box<dyn Fn(&I, &R) -> Result<(), String>>>,
}

pub trait TaskHandlerExecutorTrait<I, R> {
    // 执行一个任务块并管理后续的操作或错误处理，返回结果 R
    fn execute(&mut self, input: &I) -> Result<R, String>;
}

impl<I, R: Clone> TaskHandlerExecutorTrait<I, R> for TaskHandler<I, R> {
    fn execute(&mut self, input: &I) -> Result<R, String> {
        self.start_time = Some(Instant::now());

        println!("任务开始执行...");

        let result = loop {
            // 如果达到最大重试次数，则失败
            if self.attempts >= self.max_attempts {
                break Err("已达到最大重试次数".to_string());
            }

            self.attempts += 1; // 增加重试次数

            // 调用 before 函数（如果有）
            if let Some(before_fn) = &self.before {
                if let Err(e) = before_fn(input) {
                    println!(
                        "before 函数执行失败，第 {}/{} 次尝试。错误：{}，重试中...",
                        self.attempts, self.max_attempts, e
                    );
                    continue; // 继续重试
                }
            }

            // 调用 handle 函数
            let handle_result = (self.handle)(input);

            match handle_result {
                Ok(ref r) => {
                    // 调用 after 函数（如果有）
                    if let Some(after_fn) = &self.after {
                        if let Err(e) = after_fn(input, r) {
                            println!(
                                "after 函数执行失败，第 {}/{} 次尝试。错误：{}，重试中...",
                                self.attempts, self.max_attempts, e
                            );
                            continue; // 继续重试
                        }
                    }

                    // 如果所有函数都成功执行，退出循环
                    self.end_time = Some(Instant::now());
                    break Ok(r.clone());
                }
                Err(e) => {
                    println!(
                        "handle 函数执行失败，第 {}/{} 次尝试。错误：{}，重试中...",
                        self.attempts, self.max_attempts, e
                    );
                    continue; // 继续重试
                }
            }
        };

        // 如果任务成功或失败，则记录任务结果
        match &result {
            Ok(_) => {
                println!("任务执行成功。");
                if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
                    let duration = end.duration_since(start);
                    println!("任务耗时：{:?}", duration);
                }
            }
            Err(e) => {
                println!("任务执行失败，错误：{}", e);
            }
        }
        result
    }
}