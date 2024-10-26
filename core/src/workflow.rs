use actix::prelude::*;
use actix::Actor;

use crate::planner::WorkflowPlanner;
use crate::task::Task;
use crate::worker::Worker;
use crate::worker::WorkerMessage;

/// Workflow struct responsible for orchestrating task execution within a workflow.
pub struct Workflow {
    /// WorkflowPlanner instance to determine the next executable nodes.
    planner: WorkflowPlanner,
    /// Worker actor address responsible for executing tasks, initialized upon start.
    worker: Option<Addr<Worker>>,
}

impl Workflow {
    /// Creates a new Workflow instance with the specified planner.
    pub fn new(planner: WorkflowPlanner) -> Self {
        Self {
            planner,
            worker: None,
        }
    }

    /// Starts the workflow execution, initializing the worker if necessary.
    pub fn start_execution(&mut self, ctx: &mut Context<Self>) {
        // Initialize the Worker only when starting the execution
        if self.worker.is_none() {
            let worker_addr = Worker::new(ctx.address()).start();
            self.worker = Some(worker_addr);
        }

        // Begin with the start node in the workflow
        let start_node = self.planner.start_node.clone();
        self.dispatch_task(&start_node);
    }

    /// Dispatches a task to the Worker.
    fn dispatch_task(&self, node: &crate::node::Node) {
        if let Some(worker) = &self.worker {
            let task = Task::new(&node.id, &node.name, &node.executor_id);
            worker.do_send(WorkerMessage { task });
        } else {
            eprintln!("Worker not initialized. Cannot dispatch task.");
        }
    }

    /// Handles task completion by moving to the next nodes based on the planner's logic.
    fn on_task_completed(&mut self, task: Task, ctx: &mut Context<Self>) {
        let completed_node_id = task.id.clone();
        let next_nodes = if let Some(node) = self.planner.node_map.get(&completed_node_id) {
            self.planner.next_nodes(node, &serde_json::Value::Null)
        } else {
            vec![]
        };

        if !next_nodes.is_empty() {
            for node in next_nodes {
                self.dispatch_task(&node);
            }
        } else {
            if let Some(worker) = &self.worker {
            }
        }
    }
}

/// Actor implementation for Workflow.
impl Actor for Workflow {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Workflow started");
        // 延迟初始化 Worker 并启动任务执行
        self.start_execution(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("Workflow stopped");
    }
}

/// Message indicating a task has completed.
pub struct TaskCompleted {
  pub task: Task,
}

impl Message for TaskCompleted {
  type Result = ();
}

impl Handler<TaskCompleted> for Workflow {
  type Result = ();

  fn handle(&mut self, msg: TaskCompleted, ctx: &mut Self::Context) {
      println!("Task completed: {:?}", msg.task);
      self.on_task_completed(msg.task, ctx);
  }
}