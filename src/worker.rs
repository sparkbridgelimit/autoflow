use std::collections::HashMap;

use crate::{fetcher::Fetcher, task::Task};

pub trait Handler {
  fn handle(&self, task: &Task);
}

pub enum WorkerStatus {
  Idle,
  Busy,
}

pub struct Worker {
  pub current_task: Option<Task>,
  pub task_queue: Vec<Task>,
  pub status: WorkerStatus,
  pub manual: HashMap<String, Box<dyn Handler>>,
  pub fetcher: Fetcher,
}

