#[cfg(test)]
mod tests {
    use autoflow::fetcher::{Fetcher, LocalQueue, LocalQueueFetcher, LOCAL_QUEUE_INSTANCE};
    use autoflow::task::Task;
    use serde_json::json;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = LocalQueue::new();
        assert_eq!(queue.size(), 0);

        let task1 = Task::new(json!({"task": "Task1"}));
        let task2 = Task::new(json!({"task": "Task2"}));

        queue.enqueue(task1.clone());
        assert_eq!(queue.size(), 1);

        queue.enqueue(task2.clone());
        assert_eq!(queue.size(), 2);

        let dequeued1 = queue.dequeue();
        assert_eq!(dequeued1, Some(task1));
        assert_eq!(queue.size(), 1);

        let dequeued2 = queue.dequeue();
        assert_eq!(dequeued2, Some(task2));
        assert_eq!(queue.size(), 0);

        let dequeued3 = queue.dequeue();
        assert_eq!(dequeued3, None);
    }

    #[test]
    fn test_singleton_enqueue_dequeue() {
        let task1 = Task::new(json!({"task": "SingletonTask1"}));
        let task2 = Task::new(json!({"task": "SingletonTask2"}));

        // 入队
        {
            let mut queue = LOCAL_QUEUE_INSTANCE.lock().unwrap();
            queue.enqueue(task1.clone());
            queue.enqueue(task2.clone());
        }

        // 出队
        let fetcher = LocalQueueFetcher;
        let fetched_tasks = fetcher.fetch();
        assert_eq!(fetched_tasks, vec![task1]);

        let fetched_tasks = fetcher.fetch();
        assert_eq!(fetched_tasks, vec![task2]);

        let fetched_tasks = fetcher.fetch();
        assert!(fetched_tasks.is_empty());
    }
}
