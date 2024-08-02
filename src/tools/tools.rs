use super::{
    tool_code_interpreter::CodeInterpreter, tool_file_append::FileAppend,
    tool_file_write::FileWrite, tool_finish::Finish, tool_search::Search, ToolExector, ToolPrompt,
};
use crate::agent::response::Command;
use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::convert::TryFrom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
#[enum_dispatch(ToolExector, ToolPrompt, Debug)]
pub enum Tools {
    Search(Search),
    FileWrite(FileWrite),
    FileAppend(FileAppend),
    CodeInterpreter(CodeInterpreter),
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
                    .ok_or_else(|| anyhow!("Missing search input arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Input arg convert str failed"))?
                    .to_string();

                Ok(Tools::Search(Search::new(input)))
            }
            "file_write" => {
                let filename = command
                    .args
                    .get("filename")
                    .ok_or_else(|| anyhow!("Missing file_write filename arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Filename arg convert str failed"))?
                    .to_string();

                let content = command
                    .args
                    .get("content")
                    .ok_or_else(|| anyhow!("Missing file_write content arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Content arg convert str failed"))?
                    .to_string();

                Ok(Tools::FileWrite(FileWrite::new(filename, content)))
            }
            "file_append" => {
                let filename = command
                    .args
                    .get("filename")
                    .ok_or_else(|| anyhow!("Missing file_append filename arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Filename arg convert str failed"))?
                    .to_string();

                let content = command
                    .args
                    .get("content")
                    .ok_or_else(|| anyhow!("Missing file_append content arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Content arg convert str failed"))?
                    .to_string();

                Ok(Tools::FileAppend(FileAppend::new(filename, content)))
            }
            "code_interpreter" => {
                let language = command
                    .args
                    .get("language")
                    .ok_or_else(|| anyhow!("Missing code_interpreter language arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Language arg convert str failed"))?
                    .to_string();

                let code = command
                    .args
                    .get("code")
                    .ok_or_else(|| anyhow!("Missing code_interpreter code arg"))?
                    .as_str()
                    .ok_or_else(|| anyhow!("Code arg convert str failed"))?
                    .to_string();

                Ok(Tools::CodeInterpreter(CodeInterpreter::new(language, code)))
            }
            "finish" => {
                let result = command
                    .args
                    .get("result")
                    .ok_or_else(|| anyhow!("Missing finish result arg"))?
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
