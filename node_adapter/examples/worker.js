const { new_app, resource } = require('../pkg/node_adapter');

async function main() {
    try {
        console.log("Initializing NodeApp...");
        const app = new_app(); // 初始化 NodeApp

        // 注册任务
        app.service(
            resource("register_user").to((userData) => {
                console.log("Executing register_user...");
                if (!userData.email) {
                    throw new Error("Email is required for registration.");
                }
                console.log(`User registered: ${JSON.stringify(userData)}`);
                return { userId: "12345", ...userData };
            })
        );

        app.service(
            resource("verify_email").to(async (userId) => {
                console.log("Executing verify_email...");
                return new Promise((resolve) => {
                    setTimeout(() => {
                        console.log(`Email verified for user: ${userId}`);
                        resolve(`Verified email for user ${userId}`);
                    }, 1000);
                });
            })
        );

        app.service(
            resource("send_welcome_email").to((userId) => {
                console.log("Executing send_welcome_email...");
                console.log(`Welcome email sent to user: ${userId}`);
                return `Welcome email sent to user ${userId}`;
            })
        );

        // 模拟工作流执行
        console.log("\nStarting workflow...");
        const userData = { name: "John Doe", email: "john.doe@example.com" };

        console.log("\nStep 1: User Registration");
        const registeredUser = await app.call_task(
            "register_user",
            userData // 将 userData 传递到 call_task
        );
        console.log(`Step 1 Result: ${JSON.stringify(registeredUser)}`);

        console.log("\nStep 2: Email Verification");
        const verificationResult = await app.call_task(
            "verify_email",
            registeredUser.userId
        );
        console.log(`Step 2 Result: ${verificationResult}`);

        console.log("\nStep 3: Send Welcome Email");
        const welcomeResult = await app.call_task(
            "send_welcome_email",
            registeredUser.userId
        );
        console.log(`Step 3 Result: ${welcomeResult}`);

        console.log("\nWorkflow completed successfully!");
    } catch (error) {
        console.error("Unexpected error:", error);
    }
}

main();