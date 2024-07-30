use super::react_agent_config::ReActAgentConfig;
use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageArgs,
    },
    Client,
};

const RESPONSE_FORMAT: &str = include_str!("../template/react_response_format.txt");

#[allow(unused)]
pub struct ReActAgent {
    client: Client<OpenAIConfig>,
    system_message: ChatCompletionRequestSystemMessage,
    history: Vec<ChatCompletionRequestMessage>,
}

impl ReActAgent {
    pub fn new(question: &str, config: ReActAgentConfig) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_key(config.api_key)
            .with_api_base(config.base_url);

        let system_message = build_system_message(&config.language.to_string(), question)
            .expect("Failed to build system message");

        let client = Client::with_config(openai_config);

        Self {
            client,
            system_message,
            history: vec![],
        }
    }
}

fn build_system_message(
    language: &str,
    question: &str,
) -> Result<ChatCompletionRequestSystemMessage, OpenAIError> {
    let system_prompt = format!(
        include_str!("../template/react_system_prompt.txt"),
        language = language,
        question = question,
        response_format = RESPONSE_FORMAT,
    );

    ChatCompletionRequestSystemMessageArgs::default()
        .content(system_prompt)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react_agent() -> anyhow::Result<()> {
        let config = ReActAgentConfig::builder()
            .set_api_key("my_api_key")
            .try_set_base_url("http://localhost")?
            .try_set_language("chinese")?
            .build()?;

        let agent = ReActAgent::new("How are you?", config);

        assert!(agent.system_message.content.contains("How are you?"));

        Ok(())
    }
}
