use std::cmp::Ordering;
use chrono::{DateTime, Utc};

/// A scheduled task to be executed at a specific time.
///
/// The `Task` struct represents a scheduled task in a workflow system, 
/// with details on the task ID, its execution time, and associated 
/// workflow and trigger identifiers.
///
/// This struct supports ordering by execution time, allowing it to be 
/// used in sorted collections such as `BinaryHeap` for task scheduling 
/// where earlier tasks are prioritized.
#[derive(Debug, Eq)]
pub struct Task {
    /// Unique identifier for the task.
    pub id: String,

    /// Scheduled time for the task execution.
    pub run_at: DateTime<Utc>,

    /// Identifier of the associated workflow.
    pub workflow_id: String,

    /// Identifier of the associated trigger.
    pub trigger_id: String,
}

impl Task {
    pub fn new (id: &str, run_at: DateTime<Utc>, workflow_id: &str, trigger_id: &str) -> Self {
        Self {
            id: id.to_string(),
            run_at,
            workflow_id: workflow_id.to_string(),
            trigger_id: trigger_id.to_string(),
        }
    }
}

impl Ord for Task {
    /// Compares two tasks based on their `run_at` execution time, 
    /// ordering them to prioritize earlier tasks.
    ///
    /// This implementation reverses the ordering to ensure that tasks 
    /// with earlier `run_at` times are placed at the top of collections 
    /// such as `BinaryHeap`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_module::Task;
    /// # use chrono::Utc;
    /// # use std::cmp::Ordering;
    /// let task1 = Task { id: String::from("1"), run_at: Utc::now(), workflow_id: String::from("wf1"), trigger_id: String::from("tg1") };
    /// let task2 = Task { id: String::from("2"), run_at: Utc::now() + chrono::Duration::seconds(10), workflow_id: String::from("wf2"), trigger_id: String::from("tg2") };
    /// assert_eq!(task1.cmp(&task2), Ordering::Less);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        other.run_at.cmp(&self.run_at)
    }
}

impl PartialOrd for Task {
    /// Provides a partial comparison for tasks based on their execution time.
    ///
    /// This function delegates to the `cmp` method, enabling partial ordering
    /// which is consistent with the `Ord` implementation.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    /// Checks equality between two tasks based on their execution time.
    ///
    /// This implementation only compares the `run_at` field, considering
    /// two tasks equal if they have the same execution time, regardless
    /// of other fields.
    fn eq(&self, other: &Self) -> bool {
        self.run_at == other.run_at
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    fn create_task(id: &str, run_at: DateTime<Utc>, workflow_id: &str, trigger_id: &str) -> Task {
        Task {
            id: id.to_string(),
            run_at,
            workflow_id: workflow_id.to_string(),
            trigger_id: trigger_id.to_string(),
        }
    }

#[test]
fn test_task_ordering() {
    let now = Utc::now();
    let earlier_task = create_task("task1", now, "workflow1", "trigger1");
    let later_task = create_task("task2", now + Duration::seconds(10), "workflow2", "trigger2");

    // 使用 cmp 方法显式比较，确保符合预期的顺序
    assert_eq!(earlier_task.cmp(&later_task), Ordering::Greater, "Earlier task should be considered greater to be at the top of the heap");
    assert_eq!(later_task.cmp(&earlier_task), Ordering::Less, "Later task should be considered less to be below the earlier task in the heap");
}

    #[test]
    fn test_task_equality() {
        let now = Utc::now();
        let task1 = create_task("task1", now, "workflow1", "trigger1");
        let task2 = create_task("task2", now, "workflow2", "trigger2");

        assert_eq!(task1, task2, "Tasks with the same run_at should be equal");
    }

    #[test]
    fn test_task_sorting_in_binary_heap() {
        let now = Utc::now();
        let task1 = create_task("task1", now + Duration::seconds(30), "workflow1", "trigger1");
        let task2 = create_task("task2", now + Duration::seconds(10), "workflow2", "trigger2");
        let task3 = create_task("task3", now + Duration::seconds(20), "workflow3", "trigger3");

        let mut heap = BinaryHeap::new();
        heap.push(task1);
        heap.push(task2);
        heap.push(task3);

        // The BinaryHeap should pop tasks in ascending order of run_at
        let first_task = heap.pop().expect("Expected a task in the heap");
        assert_eq!(first_task.id, "task2", "First task should be the one with earliest run_at");

        let second_task = heap.pop().expect("Expected a task in the heap");
        assert_eq!(second_task.id, "task3", "Second task should be the one with second earliest run_at");

        let third_task = heap.pop().expect("Expected a task in the heap");
        assert_eq!(third_task.id, "task1", "Third task should be the one with latest run_at");
    }
}
