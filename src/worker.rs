use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{fetcher::Fetcher, handler2::TaskHandlerExec, task::Task};

#[derive(Debug, Clone, PartialEq)]
pub enum WorkerStatus {
    Idle,
    Busy,
}

pub struct Worker {
    // 当前执行的任务
    pub current_task: Option<Task>,

    // 任务队列
    pub task_queue: Arc<Mutex<Vec<Task>>>,

    // worker状态
    pub status: Arc<Mutex<WorkerStatus>>,

    // 任务类型到处理器的映射
    pub handlers_map: HashMap<String, Arc<dyn TaskHandlerExec + Send + Sync>>,

    // 任务获取器
    pub fetcher: Arc<dyn Fetcher + Send + Sync>,

    // 允许同时运行的个数
    pub concurrency: i32,

    // 任务执行的最大次数
    pub task_limit: usize,
}

impl Worker {
    /// 创建一个新的工人
    pub fn new(fetcher: Arc<dyn Fetcher + Send + Sync>) -> Self {
        Worker {
            current_task: None,
            task_queue: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(Mutex::new(WorkerStatus::Idle)),
            handlers_map: HashMap::new(),
            fetcher,
            concurrency: 1,
            task_limit: 0,
        }
    }

    pub fn with_limit(&mut self, task_limit: usize) {
        self.task_limit = task_limit
    }

    /// 添加任务处理器
    pub fn add_handler<T: TaskHandlerExec + 'static + Send + Sync>(
        &mut self,
        task_type: String,
        handler: T,
    ) {
        self.handlers_map.insert(task_type, Arc::new(handler));
    }

    /// 从一个 handlers 数组中学习任务处理器并添加到 handlers_map
    pub fn learn(&mut self, handlers: Vec<(String, Arc<dyn TaskHandlerExec + Send + Sync>)>) {
        for (task_type, handler) in handlers.into_iter() {
            self.handlers_map.insert(task_type, handler);
        }
    }

    /// 异步从 Fetcher 中获取任务，并放入任务队列
    pub async fn fetch(&self) {
        let fetched_tasks = self.fetcher.fetch().await;

        if fetched_tasks.is_empty() {
            println!("暂时没有任务, 继续监听");
        } else {
            let mut queue = self.task_queue.lock().await;
            for task in fetched_tasks.into_iter() {
                queue.push(task);
            }
            println!("任务已加入队列");
        }
    }

    /// 异步从任务队列中执行任务
    pub async fn execute(&self) {
        let mut queue = self.task_queue.lock().await;

        // 异步处理任务队列中的任务
        while let Some(task) = queue.pop() {
            let handler_map = self.handlers_map.clone();
            let status = self.status.clone();

            // 并发处理任务
            if let Some(handler) = handler_map.get(&task.task_type) {
                let handler = handler.clone();
                tokio::spawn(async move {
                    *status.lock().await = WorkerStatus::Busy;
                    println!("开始处理任务: {:?}", task);
                    handler.exec().await;
                    *status.lock().await = WorkerStatus::Idle;
                    println!("完成处理任务: {:?}", task);
                });
            } else {
                println!("未找到任务类型为 '{}' 的处理器。", task.task_type);
                return; // 找不到处理器时退出 while 循环
            }
        }
    }

    /// 主循环，负责交替执行 fetch 和 execute
    pub async fn run(&self) {
        let mut tasks_processed = 0;

        loop {
            self.fetch().await;
            self.execute().await;

            tasks_processed += 1;

            if tasks_processed >= self.task_limit {
                println!("已处理 {} 个任务，退出循环", tasks_processed);
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }
}
