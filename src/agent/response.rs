use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response {
    pub(crate) thoughts: Thoughts,
    pub(crate) command: Command,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Thoughts {
    pub(crate) text: String,
    pub(crate) reasoning: String,
    pub(crate) plan: String,
    pub(crate) criticism: String,
    pub(crate) speak: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Command {
    pub(crate) name: String,
    pub(crate) args: Value,
}
