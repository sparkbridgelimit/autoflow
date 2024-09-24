#[derive(Debug, Clone)]
pub struct ExecParam {
    /// 前一个 Block 执行后的数据
    pub prev_block_data: Option<String>,
    
    /// 当前 Block 的目标句柄（即连接点）
    pub target_handle: Option<String>,
    
    /// 上一个 Block 的源句柄
    pub source_handle: Option<String>,
    
    /// 下一个 Block 的断点计数
    pub next_block_breakpoint_count: Option<u32>,
    
    /// 是否恢复断点后的执行
    pub resume: bool,
}

impl ExecParam {
    /// 创建新的 ExecParam 实例
    pub fn new(
        prev_block_data: Option<String>,
        target_handle: Option<String>,
        source_handle: Option<String>,
        next_block_breakpoint_count: Option<u32>,
        resume: bool,
    ) -> Self {
        ExecParam {
            prev_block_data,
            target_handle,
            source_handle,
            next_block_breakpoint_count,
            resume,
        }
    }
}
