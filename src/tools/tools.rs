use super::{tool_finish::Finish, tool_search::Search, ToolExector};
use crate::agent::response::Command;
use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::{
    convert::TryFrom,
    fmt::{self, Debug, Write},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
#[enum_dispatch(ToolExector)]
pub enum Tools {
    Search(Search),
    Finish(Finish),
}

impl Debug for Tools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tools::Search(tool) => write!(f, "{:?}", tool),
            Tools::Finish(tool) => write!(f, "{:?}", tool),
        }
    }
}

impl Tools {
    pub fn commands() -> Result<String> {
        let mut output = String::new();

        for tool in Tools::iter() {
            match tool {
                Tools::Search(tool) => writeln!(output, "- {}", tool.command())?,
                Tools::Finish(tool) => writeln!(output, "- {}", tool.command())?,
            }
        }

        Ok(output)
    }

    pub fn resources() -> Result<String> {
        let mut output = String::new();

        for tool in Tools::iter() {
            match tool {
                Tools::Search(tool) => writeln!(output, "- {}", tool.resource())?,
                Tools::Finish(_) => {}
            }
        }

        Ok(output)
    }
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
