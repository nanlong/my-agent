use super::{tool_do_nothing::DoNothing, tool_search::Search};
use crate::agent::response::Command;
use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display, Write},
};
use strum::{EnumIter, IntoEnumIterator};

#[enum_dispatch]
pub trait ToolExecute {
    async fn execute(&self) -> Result<String>;
}

#[derive(EnumIter)]
#[enum_dispatch(ToolExecute)]
pub enum Tools {
    Search(Search),
    DoNothing(DoNothing),
}

impl Display for Tools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tools::Search(tool) => write!(f, "{}", tool),
            Tools::DoNothing(tool) => write!(f, "{}", tool),
        }
    }
}

impl Debug for Tools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tools::Search(tool) => write!(f, "{:?}", tool),
            Tools::DoNothing(tool) => write!(f, "{:?}", tool),
        }
    }
}

impl Tools {
    pub fn description() -> Result<String> {
        let mut output = String::new();

        for (index, tool) in Tools::iter().enumerate() {
            let index = index + 1;

            match tool {
                Tools::Search(tool) => writeln!(output, "{}. {}", index, tool)?,
                Tools::DoNothing(tool) => writeln!(output, "{}. {}", index, tool)?,
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
                    .to_string();

                Ok(Tools::Search(Search::new(input)))
            }
            "do_nothing" => Ok(Tools::DoNothing(DoNothing)),
            _ => Err(anyhow!("Unknown command")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_description() -> Result<()> {
        let mut output = String::new();

        for (index, tool) in Tools::iter().take(2).enumerate() {
            let index = index + 1;

            match tool {
                Tools::Search(tool) => writeln!(output, "{}. {}", index, tool)?,
                Tools::DoNothing(tool) => writeln!(output, "{}. {}", index, tool)?,
            }
        }

        assert_eq!(
            output,
            "1. Search : \"search\", args: \"input\": \"<search content>\"\n2. Do Nothing: \"do_nothing\", args:\n"
        );

        Ok(())
    }
}