const { run_tests } = require('../pkg/node_adapter');

async function main() {
  try {
    console.log("Running tests...");
    const results = await run_tests(); // 获取测试结果数组
    results.forEach((result, index) => {
      console.log(`Test ${index + 1}: ${result}`);
    });
    console.log("All tests completed!");
  } catch (error) {
    console.error("Error during tests:", error);
  }
}

main();