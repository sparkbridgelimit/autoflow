const { Worker } = require('../pkg/node_adapter');

async function main() {
    const worker = new Worker();

    // 注册任务
    worker.register_task("task_1", async () => {
        console.log("Executing task_1...");
        return new Promise(resolve => setTimeout(() => resolve(10), 1000));
    });

    worker.register_task("task_2", async (input) => {
        console.log(`Executing task_2 with input: ${input}`);
        return new Promise(resolve => setTimeout(() => resolve(input * 2), 1000));
    });

    worker.register_task("task_3", async (input) => {
        console.log(`Executing task_3 with input: ${input}`);
        return new Promise(resolve => setTimeout(() => resolve(input + 5), 1000));
    });

    console.log("Starting task chain execution...");
    console.time("Task Chain Execution Time");

    // 执行任务链并记录每个任务的时间和结果
    console.time("Task 1 Execution Time");
    const result1 = await worker.execute_async("task_1", null);
    console.timeEnd("Task 1 Execution Time");
    console.log("Task 1 Result:", result1);

    console.time("Task 2 Execution Time");
    const result2 = await worker.execute_async("task_2", result1);
    console.timeEnd("Task 2 Execution Time");
    console.log("Task 2 Result:", result2);

    console.time("Task 3 Execution Time");
    const result3 = await worker.execute_async("task_3", result2);
    console.timeEnd("Task 3 Execution Time");
    console.log("Task 3 Result:", result3);

    console.timeEnd("Task Chain Execution Time");
    console.log("Final Task Chain Result:", result3);
}

main();