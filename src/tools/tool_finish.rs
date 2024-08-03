use super::{ToolExector, ToolPrompt};
use anyhow::{anyhow, Result};
use async_openai::{
    error::OpenAIError,
    types::{
        ChatCompletionTool, ChatCompletionToolArgs, ChatCompletionToolType, FunctionCall,
        FunctionObjectArgs,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
        f.debug_struct("Finish")
            .field("result", &self.result)
            .finish()
    }
}

impl TryFrom<Finish> for ChatCompletionTool {
    type Error = OpenAIError;

    fn try_from(_finish: Finish) -> Result<Self, Self::Error> {
        ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("finish")
                    .description("完成用户的任务目标")
                    .parameters(json!({
                        "type": "object",
                        "properties": {
                            "result": {
                                "type": "string",
                                "description": "最终结果",
                            }
                        },
                        "required": ["result"],
                    }))
                    .build()?,
            )
            .build()
    }
}

#[derive(Serialize, Deserialize)]
struct FinishArgs {
    result: String,
}

impl TryFrom<FunctionCall> for Finish {
    type Error = anyhow::Error;

    fn try_from(call: FunctionCall) -> Result<Self, Self::Error> {
        if call.name == "finish" {
            let args: FinishArgs = serde_json::from_str(&call.arguments)?;
            Ok(Finish::new(args.result))
        } else {
            Err(anyhow!("Invalid function call: {:?}", call))
        }
    }
}
