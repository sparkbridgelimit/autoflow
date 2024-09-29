use std::{collections::VecDeque, sync::Mutex};

use async_trait::async_trait;
use once_cell::sync::Lazy;

use crate::task::Task;


#[async_trait]
pub trait Fetcher {
    async fn fetch(&self) -> Vec<Task>;
}

pub struct LocalQueue<T> {
    vec: VecDeque<T>,
}

impl<T> LocalQueue<T> {
    // 创建一个新的 LocalQueue
    pub fn new() -> Self {
        LocalQueue {
            vec: VecDeque::new(),
        }
    }
    // 获取队列的大小
    pub fn size(&self) -> usize {
        self.vec.len()
    }

    // 入队操作
    pub fn enqueue(&mut self, e: T) {
        self.vec.push_back(e);
    }

    // 出队操作
    pub fn dequeue(&mut self) -> Option<T> {
        self.vec.pop_front()
    }
}

// 实现 LocalQueue 的单例模式，专门用于 Task 类型
pub static LOCAL_QUEUE_INSTANCE: Lazy<Mutex<LocalQueue<Task>>> =
    Lazy::new(|| Mutex::new(LocalQueue::new()));

pub struct LocalQueueFetcher;

#[async_trait]
impl Fetcher for LocalQueueFetcher {
    async fn fetch(&self) -> Vec<Task> {
        let mut queue = LOCAL_QUEUE_INSTANCE.lock().unwrap();
        if let Some(task) = queue.dequeue() {
            vec![task]
        } else {
            vec![]
        }
    }
}
