use super::{tool_finish::Finish, tool_search::Search, ToolExector, ToolPrompt};
use crate::agent::response::Command;
use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::convert::TryFrom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
#[enum_dispatch(ToolExector, ToolPrompt)]
pub enum Tools {
    Search(Search),
    Finish(Finish),
}

impl TryFrom<Command> for Tools {
    type Error = anyhow::Error;

    fn try_from(command: Command) -> Result<Self> {
        match command.name.as_str() {
            "search" => {
                let input = command
                    .args
                    .get("input")
                    .ok_or_else(|| anyhow!("Missing input"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Input arg convert str failed"))?
                    .to_string();

                Ok(Tools::Search(Search::new(input)))
            }
            "finish" => {
                let result = command
                    .args
                    .get("result")
                    .ok_or_else(|| anyhow!("Missing result"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Result arg convert str failed"))?
                    .to_string();

                Ok(Tools::Finish(Finish::new(result)))
            }
            _ => Err(anyhow!("Unknown command")),
        }
    }
}

impl Tools {
    pub fn commands() -> Result<String> {
        let output = Tools::iter()
            .map(|tool| format!("- {}", tool.command()))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(output)
    }

    pub fn resources() -> Result<String> {
        let output = Tools::iter()
            .filter_map(|tool| match tool {
                Tools::Finish(_) => None,
                _ => Some(format!("- {}", tool.resource())),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(output)
    }
}
