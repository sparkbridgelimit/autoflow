const { Worker } = require('../pkg/node_adapter');

async function main() {
    const worker = new Worker();

    // 注册多个异步任务
    worker.register_task("task_1", async () => {
        return new Promise(resolve => setTimeout(() => resolve("Task 1 completed!"), 2000));
    });

    worker.register_task("task_2", async () => {
        return new Promise(resolve => setTimeout(() => resolve("Task 2 completed!"), 1000));
    });

    worker.register_task("task_3", async () => {
        return new Promise(resolve => setTimeout(() => resolve("Task 3 completed!"), 3000));
    });

    // 并发执行任务
    console.time("Concurrent Tasks Execution Time");
    const results = await Promise.all([
        worker.execute_async("task_1"),
        worker.execute_async("task_2"),
        worker.execute_async("task_3"),
    ]);
    console.timeEnd("Concurrent Tasks Execution Time");

    console.log("Results:", results);
}

main();