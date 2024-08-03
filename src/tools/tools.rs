use super::{
    tool_file_write::FileWrite, tool_finish::Finish, tool_search::Search, ToolExector, ToolPrompt,
};
use anyhow::{anyhow, Result};
use async_openai::types::{ChatCompletionTool, FunctionCall};
use enum_dispatch::enum_dispatch;
use std::convert::TryFrom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
#[enum_dispatch(ToolExector, ToolPrompt, Debug, Default)]
pub enum Tools {
    Search(Search),
    FileWrite(FileWrite),
    Finish(Finish),
}

impl TryFrom<FunctionCall> for Tools {
    type Error = anyhow::Error;

    fn try_from(call: FunctionCall) -> Result<Self, Self::Error> {
        match call.name.as_ref() {
            "search" => Ok(Tools::Search(call.try_into()?)),
            "file_write" => Ok(Tools::FileWrite(call.try_into()?)),
            "finish" => Ok(Tools::Finish(call.try_into()?)),
            _ => Err(anyhow!("Unknown tool")),
        }
    }
}

impl Tools {
    pub fn list() -> Vec<ChatCompletionTool> {
        Tools::iter()
            .filter_map(|tool| match tool {
                Tools::Search(tool) => tool.try_into().ok(),
                Tools::FileWrite(tool) => tool.try_into().ok(),
                Tools::Finish(tool) => tool.try_into().ok(),
                // _ => None,
            })
            .collect::<Vec<_>>()
    }
}
