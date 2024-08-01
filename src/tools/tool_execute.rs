use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait ToolExector {
    async fn execute(&self) -> Result<String>;
}
