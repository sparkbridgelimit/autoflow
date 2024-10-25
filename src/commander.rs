use async_trait::async_trait;

pub struct Context {
    user_id: Option<String>,
    request_id: Option<String>,
}

#[async_trait]
pub trait FromContext: Sized {
    async fn from_context(ctx: &Context) -> Result<Self, &'static str>;
}

pub struct UserInfo {
    pub user_id: String,
}

#[async_trait]
impl FromContext for UserInfo {
    async fn from_context(ctx: &Context) -> Result<Self, &'static str> {
        let user_id = ctx.user_id.clone().ok_or("user_id not found")?;
        Ok(UserInfo { user_id })
    }
}

pub struct RequestInfo {
    pub request_id: String,
}

#[async_trait]
impl FromContext for RequestInfo {
    async fn from_context(ctx: &Context) -> Result<Self, &'static str> {
        let request_id = ctx.request_id.clone().ok_or("request_id not found")?;
        Ok(RequestInfo { request_id })
    }
}

#[async_trait]
pub trait CommandHandler {
    async fn call(&self, ctx: &Context) -> Result<(), &'static str>;
}

// 定义命令者结构体
struct Commander {
    commands: Vec<Box<dyn CommandHandler + Send + Sync>>,
}

impl Commander {
    fn new() -> Self {
        Commander { commands: vec![] }
    }

    fn add_command<F>(&mut self, func: F)
    where
        F: CommandHandler + Send + Sync + 'static,
    {
        self.commands.push(Box::new(func));
    }

    async fn execute(&self, ctx: &Context) -> Result<(), &'static str> {
        if let Some(command) = self.commands.first() {
            command.call(ctx).await
        } else {
            Err("Command not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub async fn demo() -> Result<(), &'static str> {
        Ok(())
    }


    #[tokio::test] // 使用 tokio 异步运行时测试
    async fn test_commander_demo() {
        let mut commander = Commander::new();

        // 注册命令
        commander.add_command(demo);

        // 创建一个上下文对象
        let ctx = Context {
            user_id: Some("user123".to_string()),
            request_id: Some("req456".to_string()),
        };

        // 执行命令并传入上下文，检查是否成功
        let result = commander.execute(&ctx).await;
        assert!(result.is_ok(), "Commander failed to execute demo command");
    }
}
