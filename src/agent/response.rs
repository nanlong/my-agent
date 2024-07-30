use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub thoughts: Thoughts,
    pub command: Command,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thoughts {
    pub text: String,
    pub reasoning: String,
    pub plan: String,
    pub criticism: String,
    pub speak: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub args: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_response() -> anyhow::Result<()> {
        let response = r#"{
            "thoughts": {
                "text": "I am a test response",
                "reasoning": "I am a test reasoning",
                "plan": "I am a test plan",
                "criticism": "I am a test criticism",
                "speak": "I am a test speak"
            },
            "command": {
                "name": "test_command",
                "args": {
                    "arg1": "value1",
                    "arg2": "value2"
                }
            }
        }"#;

        let response: Response = serde_json::from_str(response)?;

        assert_eq!(response.thoughts.text, "I am a test response");
        assert_eq!(response.thoughts.reasoning, "I am a test reasoning");
        assert_eq!(response.thoughts.plan, "I am a test plan");
        assert_eq!(response.thoughts.criticism, "I am a test criticism");
        assert_eq!(response.thoughts.speak, "I am a test speak");

        assert_eq!(response.command.name, "test_command");
        assert_eq!(response.command.args["arg1"], "value1");
        assert_eq!(response.command.args["arg2"], "value2");

        Ok(())
    }
}
