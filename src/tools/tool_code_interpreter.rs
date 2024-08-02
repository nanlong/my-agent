use super::{ToolExector, ToolPrompt};
use anyhow::Result;
use pyo3::prelude::*;
use std::fmt::{self, Debug};

#[derive(Default)]
pub struct CodeInterpreter {
    language: String,
    code: String,
}

impl CodeInterpreter {
    pub fn new(language: String, code: String) -> Self {
        CodeInterpreter { language, code }
    }
}

impl ToolExector for CodeInterpreter {
    async fn execute(&self) -> Result<String> {
        match self.language.as_str() {
            "Python" => run_python_code(&self.code),
            _ => Err(anyhow::anyhow!("不支持的编程语言")),
        }
    }
}

impl ToolPrompt for CodeInterpreter {
    fn command(&self) -> String {
        r#"{
            "name": "code_interpreter",
            "description": "执行代码",
            "args": [
                {
                    "name": "language",
                    "type": "string",
                    "description": "编程语言，目前支持的语言有：Python"
                },
                {
                    "name": "code",
                    "type": "string",
                    "description": "要执行的代码内容"
                }
            ]
        }"#
        .to_string()
    }

    fn resource(&self) -> String {
        "执行代码获得结果".to_string()
    }
}

impl Debug for CodeInterpreter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodeInterpreter")
            .field("language", &self.language)
            .field("code", &"...")
            .finish()
    }
}

fn run_python_code(code: &str) -> Result<String> {
    let result = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let result = py.eval_bound(code, None, None)?.extract()?;
        Ok(result)
    })?;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_python_code() -> Result<()> {
        let result = run_python_code("1 + 1")?;
        assert_eq!(result, "2");

        Ok(())
    }

    #[tokio::test]
    async fn test_code_interpreter() -> Result<()> {
        let code_interpreter = CodeInterpreter::new("Python".to_string(), "1 + 1".to_string());
        let result = code_interpreter.execute().await?;
        assert_eq!(result, "2");

        Ok(())
    }
}
