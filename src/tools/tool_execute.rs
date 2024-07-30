use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait ToolExecute {
    async fn execute(&self) -> Result<String>;
}
