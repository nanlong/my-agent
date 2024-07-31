use super::ToolExecute;
use anyhow::Result;
use std::fmt::{self, Debug, Display};

#[derive(Default)]
pub struct Finish {
    #[allow(dead_code)]
    result: String,
}

impl ToolExecute for Finish {
    async fn execute(&self) -> Result<String> {
        Ok("Done".to_string())
    }
}

impl Debug for Finish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Finish").finish()
    }
}

impl Display for Finish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description =
            r#"Called when the task is complete: "finish", args: "result": "<final answer>""#;
        write!(f, "{}", description)
    }
}
