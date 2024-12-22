const { Worker } = require('../pkg/node_adapter');

async function main() {
    const worker = new Worker();
    console.log("Starting task...");
    worker.start_task();
    console.log("Task started. Waiting for completion...");
}

main();