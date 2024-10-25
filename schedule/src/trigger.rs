use std::str::FromStr;

use chrono::{DateTime, Utc};
use cron::Schedule;

pub struct Trigger {
    pub id: String,
    pub cron_expr: String,
    pub workflow_id: String,
}

impl Trigger {
    /// Creates a new `Trigger` instance.
    pub fn new(id: &str, cron_expr: &str, workflow_id: &str) -> Self {
        Self {
            id: id.to_string(),
            cron_expr: cron_expr.to_string(),
            workflow_id: workflow_id.to_string(),
        }
    }

    /// Generates the next run times within the specified time window.
    ///
    /// # Arguments
    ///
    /// * `start` - The start of the time window.
    /// * `end` - The end of the time window.
    ///
    /// # Returns
    ///
    /// * `Vec<DateTime<Utc>>` - A vector of `DateTime<Utc>` representing the next run times within the window.
    pub fn next_run_times(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<DateTime<Utc>> {
        let mut times = Vec::new();

        let schedule = match Schedule::from_str(&self.cron_expr) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Invalid cron expression: {}", self.cron_expr);
                return times;
            }
        };

        for datetime in schedule.upcoming(Utc).take_while(|&dt| dt < end) {
            if datetime >= start && datetime < end {
                times.push(datetime);
            }
        }

        times
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
    fn test_next_run_times_with_valid_cron() {
        // 设置触发器和时间窗口
        let trigger = create_trigger("trigger1", "*/5 * * * * *", "workflow1");
        let start = Utc::now();
        let end = start + Duration::seconds(20); // 未来20秒的时间窗口

        // 获取运行时间
        let times = trigger.next_run_times(start, end);

        // 验证生成的时间数量和边界
        assert!(!times.is_empty(), "Expected run times to be generated.");
        assert!(times.len() >= 1, "Expected at least one run time within the time window.");

        for time in &times {
            assert!(*time >= start && *time < end,
                    "Run time is not within the specified time window.");
        }
    }

    #[test]
    fn test_next_run_times_with_invalid_cron() {
        // 设置无效的cron表达式触发器
        let trigger = create_trigger("invalid_trigger", "invalid cron expr", "workflow1");
        let start = Utc::now();
        let end = start + Duration::seconds(20);

        // 获取运行时间
        let times = trigger.next_run_times(start, end);

        // 无效cron表达式不应生成任何时间
        assert!(times.is_empty(), "Expected no run times for an invalid cron expression.");
    }

    #[test]
    fn test_next_run_times_with_edge_of_time_window() {
        // 设置触发器，每5秒执行一次
        let trigger = create_trigger("trigger2", "*/5 * * * * *", "workflow2");
        let start = Utc::now();
        let end = start + Duration::seconds(10); // 未来10秒的时间窗口

        // 获取运行时间
        let times = trigger.next_run_times(start, end);

        // 验证生成的时间是否在指定的窗口范围内
        assert!(!times.is_empty(), "Expected run times to be generated.");
        for time in &times {
            assert!(*time >= start && *time < end,
                    "Run time is not within the specified time window.");
        }

        // 验证是否包含正确的时间数量
        assert!(times.len() <= 2, "Expected at most two run times within a 10-second window.");
    }
}
