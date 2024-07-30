use anyhow::Result;
use serde::Serializer;
use std::fmt::Display;

struct Tools {
    tools: Vec<Box<dyn ToolExecute>>,
}

trait ToolExecute {
    fn execute(&self, input: String) -> Result<String>;

    // example:
    //  Google Search: "google", args: "input": "<search>"
    fn description(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::fmt::{self, Display};

    #[derive(Serialize)]
    struct TestTool {
        name: String,
    }

    impl Display for TestTool {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }
}
