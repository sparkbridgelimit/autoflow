use std::{collections::HashMap, sync::Arc};

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
    pub task_queue: Vec<Task>,

    // 工人的状态
    pub status: WorkerStatus,

    // 任务类型到处理器的映射
    pub handlers_map: HashMap<String, Arc<dyn TaskHandlerExec + Send + Sync>>,

    // 任务获取器
    pub fetcher: Box<dyn Fetcher + Send + Sync>,

    // 允许同时运行的个数
    pub concurrency: i32,
}

impl Worker {
    /// 创建一个新的工人
    pub fn new(fetcher: Box<dyn Fetcher + Send + Sync>) -> Self {
        Worker {
            current_task: None,
            task_queue: Vec::new(),
            status: WorkerStatus::Idle,
            handlers_map: HashMap::new(),
            fetcher,
            concurrency: 1,
        }
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

    /// run 运行任务
    pub fn run(&mut self) {
        loop {
            // 从 Fetcher 获取任务
            let fetched_tasks = self.fetcher.fetch();

            if fetched_tasks.is_empty() {
                println!("暂时没有任务, 继续监听");
                continue;
            }

            // 将任务加入任务队列，最多加入 `max_tasks` 个
            for task in fetched_tasks.into_iter() {
                self.task_queue.push(task);
            }

            // 处理任务队列中的任务
            while let Some(task) = self.task_queue.pop() {
                if let Some(handler) = self.handlers_map.get(&task.task_type) {
                    self.status = WorkerStatus::Busy;
                    println!("开始处理任务: {:?}", task);
                    handler.exec();
                    self.status = WorkerStatus::Idle;
                    println!("完成处理任务: {:?}", task);
                } else {
                    println!("未找到任务类型为 '{}' 的处理器。", task.task_type);
                }
            }
        }
    }
}
