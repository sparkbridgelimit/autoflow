use std::time::Duration;

use actix::{Actor, ActorContext, Addr, Context, Handler, Message};

use crate::{task::Task, workflow::{TaskCompleted, Workflow}};

/// Worker struct responsible for executing tasks within a workflow.
pub struct Worker {
    /// Address of the Workflow actor to notify upon task completion.
    workflow: Addr<Workflow>,
}

impl Worker {
    /// Creates a new Worker instance with a reference to the Workflow actor.
    pub fn new(workflow: Addr<Workflow>) -> Self {
        Self { workflow }
    }

    /// Starts the worker to process incoming tasks.
    pub fn execute(&self, task: Task) {
        println!("Worker received a new task to execute: {:?}", task);
        // Simulate task execution with a delay
        let workflow: Addr<Workflow> = self.workflow.clone();
        actix::spawn(async move {
            // Simulated task execution duration
            actix::clock::sleep(Duration::from_secs(2)).await;
            // Notify workflow of task completion
            workflow.do_send(TaskCompleted(task));
        });
    }
}

impl Actor for Worker {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Worker started");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("Worker stopped");
    }
}

/// Message sent to Worker to execute a specific task.
pub struct WorkerMessage {
    pub task: Task,
}

impl Message for WorkerMessage {
    type Result = ();
}

impl Handler<WorkerMessage> for Worker {
    type Result = ();

    fn handle(&mut self, msg: WorkerMessage, _: &mut Self::Context) {
        println!("Worker is processing task: {:?}", msg.task);
        // 启动任务处理流程
        self.execute(msg.task);
    }
}

/// Message to start the Worker.
pub struct Start;

impl Message for Start {
    type Result = ();
}

impl Handler<Start> for Worker {
    type Result = ();

    fn handle(&mut self, _: Start, _: &mut Self::Context) {
        println!("Worker started processing tasks.");
    }
}

/// Message to stop the Worker.
pub struct Stop;

impl Message for Stop {
    type Result = ();
}

impl Handler<Stop> for Worker {
    type Result = ();

    fn handle(&mut self, _: Stop, ctx: &mut Self::Context) {
        println!("Worker is stopping.");
        ctx.stop();
    }
}
