use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait ToolExector {
    async fn execute(&self) -> Result<String>;
}

#[enum_dispatch]
pub trait ToolPrompt {
    fn command(&self) -> String;
    fn resource(&self) -> String;
}
