use std::{collections::HashMap, sync::Arc};

use tokio::sync::mpsc;
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
    pub async fn fetch(&self, tx: &mpsc::Sender<Task>) {
        let fetched_tasks = self.fetcher.fetch().await;

        if fetched_tasks.is_empty() {
            println!("暂时没有任务, 继续监听");
        } else {
            for task in fetched_tasks {
                if let Err(_) = tx.send(task).await {
                    println!("任务发送失败");
                }
            }
            println!("任务已加入队列");
        }
    }

    /// 主循环，负责交替执行 fetch 和 execute
    pub async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel(100);

        let mut tasks_processed = 0;

        loop {
            let fetched_tasks = self.fetcher.fetch().await;
            for task in fetched_tasks {
                if tx.send(task).await.is_err() {
                    println!("任务发送失败");
                }
            }

            // 异步接收任务并执行
            while let Some(task) = rx.recv().await {
                let handler_map = self.handlers_map.clone();
                let status = self.status.clone();

                // 根据任务类型执行对应的处理器
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
                }

                tasks_processed += 1;
                if tasks_processed >= self.task_limit {
                    println!("已处理 {} 个任务，退出循环", tasks_processed);
                    return; // 达到任务限制，退出主循环
                }
            }
        }
    }
}
