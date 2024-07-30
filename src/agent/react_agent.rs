use super::react_agent_config::ReActAgentConfig;
use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_stream::stream;
use futures::Stream;
use std::pin::Pin;

const RESPONSE_FORMAT: &str = include_str!("../template/react_response_format.txt");

type MessageStream =
    Pin<Box<dyn Stream<Item = Result<ChatCompletionRequestAssistantMessage>> + Send>>;

#[allow(unused)]
pub struct ReActAgent {
    client: Client<OpenAIConfig>,
    config: ReActAgentConfig,
}

impl ReActAgent {
    pub fn new(config: ReActAgentConfig) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_key(config.api_key.clone())
            .with_api_base(config.base_url.clone());

        let client = Client::with_config(openai_config);

        Self { client, config }
    }

    pub async fn invoke(self, question: &str) -> Result<MessageStream> {
        let language = self.config.language.to_string();

        let system_message =
            build_system_message(question, &language).expect("Failed to build system message");
        let mut history: Vec<ChatCompletionRequestMessage> = Vec::new();
        let max_steps = self.config.max_steps;

        println!("Question: {}", question);

        let stream = stream! {
            for _step in 1..=max_steps {
                let messages = [
                    vec![system_message.clone().into()],
                    history.clone(),
                ].into_iter().flatten().collect::<Vec<_>>();

                let request = CreateChatCompletionRequestArgs::default()
                    .model(&self.config.model)
                    .messages(messages)
                    .temperature(0.3_f32)
                    .build()?;

                let ret = self.client.chat().create(request).await?;
                let response = ret.choices[0].message.content.clone().unwrap_or_default();
                let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
                        .content(response)
                        .build()?;

                let user_message = ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Command result: {} \n {}", "success", RESPONSE_FORMAT))
                    .build()?;

                history.push(assistant_message.clone().into());
                history.push(user_message.clone().into());


                yield Ok(assistant_message)
            }
        };

        Ok(Box::pin(stream))
    }
}

fn build_system_message(
    question: &str,
    language: &str,
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
            .set_model("moonshot-v1-8k")
            .try_set_base_url("http://localhost")?
            .build()?;

        let agent = ReActAgent::new(config.clone());

        assert_eq!(agent.config, config);
        Ok(())
    }
}
