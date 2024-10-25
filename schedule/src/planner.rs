use chrono::{DateTime, Utc};

use crate::{task::Task, trigger::Trigger};

/// Planner struct responsible for generating task schedules based on triggers.
pub struct Planner {
    /// List of triggers, each representing a scheduled workflow.
    triggers: Vec<Trigger>,
}

impl Planner {
    /// Creates a new `Planner` instance with a list of triggers.
    pub fn new(triggers: Vec<Trigger>) -> Self {
        Self { triggers }
    }

    /// Generates a batch of tasks based on triggers and the specified time window.
    ///
    /// # Arguments
    ///
    /// * `end` - The end of the time window, tasks are generated from now until this time.
    ///
    /// # Returns
    ///
    /// * `Vec<Task>` - A vector of `Task` objects scheduled to run within the specified time window.
    pub fn generate_tasks(&self, end: DateTime<Utc>) -> Vec<Task> {
        let mut tasks = Vec::new();
        let now = Utc::now();

        for trigger in &self.triggers {
            // 使用 Trigger 的 next_run_times 方法获取时间窗口内的所有执行时间
            for run_at in trigger.next_run_times(now, end) {
                // Create a new Task
                let task = Task {
                    id: format!("{}-{}", trigger.id, run_at.timestamp()), // Unique task ID
                    run_at,
                    workflow_id: trigger.workflow_id.clone(),
                    trigger_id: trigger.id.clone(),
                };
                tasks.push(task);
            }
        }

        tasks
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_trigger(id: &str, cron_expr: &str, workflow_id: &str) -> Trigger {
        Trigger::new(id, cron_expr, workflow_id)
    }

    #[test]
    fn test_generate_tasks_with_valid_triggers() {
        // 设置触发器列表和时间窗口
        let triggers = vec![
            create_trigger("trigger1", "*/5 * * * * *", "workflow1"),
            create_trigger("trigger2", "*/10 * * * * *", "workflow2"),
        ];
        let planner = Planner::new(triggers);
        let future_time = Utc::now() + Duration::seconds(20); // 未来20秒的时间窗口

        // 生成任务
        let tasks = planner.generate_tasks(future_time);

        // 检查任务是否生成，并验证每个任务的时间边界
        assert!(!tasks.is_empty(), "Expected tasks to be generated.");
        let now = Utc::now();
        for task in &tasks {
            assert!(task.run_at >= now && task.run_at < future_time,
                    "Task run_at time is not within the specified time window.");
        }
    }

    #[test]
    fn test_generate_tasks_with_empty_triggers() {
        // 创建一个空的触发器列表
        let planner = Planner::new(Vec::new());
        let future_time = Utc::now() + Duration::seconds(20); // 未来20秒的时间窗口

        // 生成任务
        let tasks = planner.generate_tasks(future_time);

        // 验证任务列表为空
        assert!(tasks.is_empty(), "Expected no tasks to be generated for empty trigger list.");
    }

    #[test]
    fn test_generate_tasks_with_multiple_triggers() {
        // 设置多个触发器，每个触发器有不同的cron表达式
        let triggers = vec![
            create_trigger("trigger1", "*/5 * * * * *", "workflow1"),
            create_trigger("trigger2", "*/10 * * * * *", "workflow2"),
            create_trigger("trigger3", "*/15 * * * * *", "workflow3"),
        ];
        let planner = Planner::new(triggers);
        let future_time = Utc::now() + Duration::seconds(20); // 未来20秒的时间窗口

        // 生成任务
        let tasks = planner.generate_tasks(future_time);

        // 验证是否生成任务，且所有任务的时间均在时间窗口内
        assert!(!tasks.is_empty(), "Expected tasks to be generated for multiple triggers.");
        let now = Utc::now();
        for task in &tasks {
            assert!(task.run_at >= now && task.run_at < future_time,
                    "Task run_at time is not within the specified time window.");
            assert!(task.trigger_id == "trigger1" || task.trigger_id == "trigger2" || task.trigger_id == "trigger3",
                    "Unexpected trigger_id in generated task.");
        }

        // 验证每个触发器的任务数量是否合理
        let task_counts = tasks.iter().filter(|task| task.trigger_id == "trigger1").count();
        assert!(task_counts > 0, "Expected tasks from trigger1.");
    }
}
