use super::{ToolExector, ToolPrompt};
use anyhow::Result;
use std::fmt::{self, Debug};

#[derive(Default)]
pub struct Finish {
    result: String,
}

impl Finish {
    pub fn new(result: String) -> Self {
        Self { result }
    }
}

impl ToolExector for Finish {
    async fn execute(&self) -> Result<String> {
        Ok(self.result.clone())
    }
}

impl ToolPrompt for Finish {
    fn command(&self) -> String {
        r#"{
            "name": "finish",
            "description": "完成用户的任务目标",
            "args": [
                {
                    "name": "result",
                    "type": "string",
                    "description": "最终结果"
                }
            ]
        }"#
        .to_string()
    }

    fn resource(&self) -> String {
        "完成用户的任务目标".to_string()
    }
}

impl Debug for Finish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Finish").finish()
    }
}
