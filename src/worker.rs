use crate::fetcher::Fetcher;


pub struct Worker {
  // 当前执行的任务
  // pub current_task: Option<Box<dyn Task>>,

  // 任务队列
  // pub task_queue: Vec<Box<dyn Task>>,

  // 工人的状态
  // pub status: WorkerStatus,

  // 任务类型到处理器的映射
  // pub handlers: HashMap<String, Box<dyn TaskHandler>>,

  // 任务获取器
  pub fetcher: Box<dyn Fetcher>,
}