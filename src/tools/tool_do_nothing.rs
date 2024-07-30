use super::ToolExecute;
use anyhow::Result;
use std::fmt::{self, Debug, Display};

#[derive(Default)]
pub struct DoNothing;

impl Display for DoNothing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = r#"Do Nothing: "do_nothing", args:"#;
        write!(f, "{}", description)
    }
}

impl Debug for DoNothing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoNothing").finish()
    }
}

impl ToolExecute for DoNothing {
    async fn execute(&self) -> Result<String> {
        Ok("Do nothing".to_string())
    }
}
