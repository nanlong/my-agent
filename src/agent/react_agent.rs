use super::{response::Response, ReActAgentConfig};
use crate::tools::{ToolExecute, Tools};
use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    },
    Client,
};
use async_stream::stream;
use futures::Stream;
use std::pin::Pin;

const RESPONSE_FORMAT: &str = include_str!("../template/react_response_format.txt");

type MessageStream = Pin<Box<dyn Stream<Item = Result<ChatCompletionRequestMessage>> + Send>>;

#[derive(Clone)]
pub struct ReActAgent {
    client: Client<OpenAIConfig>,
    config: ReActAgentConfig,
}

impl ReActAgent {
    pub fn new(config: ReActAgentConfig) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_key(config.api_key.as_str())
            .with_api_base(config.base_url.as_str());

        let client = Client::with_config(openai_config);

        Self { client, config }
    }

    pub async fn invoke(self, question: &str) -> Result<MessageStream> {
        let question = question.to_string();
        let system_message = self.build_system_message(&question)?;
        let mut history: Vec<ChatCompletionRequestMessage> = Vec::new();

        let stream = stream! {
            let user_message = ChatCompletionRequestUserMessageArgs::default()
                .content(question)
                .build()?;

            yield Ok(user_message.into());

            'outer: for _step in 1..=self.config.max_steps {
                let response = self.planning(system_message.clone(), history.clone()).await?;

                for choice in response.choices {
                    if let Some(assistant_prompt) = choice.message.content {
                        let response: Response = serde_json::from_str(&assistant_prompt)?;

                        let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
                            .content(assistant_prompt)
                            .build()?;

                        history.push(assistant_message.clone().into());
                        yield Ok(assistant_message.into());

                        if response.command.name == "finish" {
                            break 'outer;
                        }

                        let tool: Tools = response.command.try_into()?;
                        let command_result = tool.execute().await?;
                        let user_prompt = format!("Command result: {}\n{}", command_result, RESPONSE_FORMAT);

                        let user_message = ChatCompletionRequestUserMessageArgs::default()
                            .content(user_prompt)
                            .build()?;

                        history.push(user_message.clone().into());
                        yield Ok(user_message.into());
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    async fn planning(
        &self,
        system_message: ChatCompletionRequestSystemMessage,
        history: Vec<ChatCompletionRequestMessage>,
    ) -> Result<CreateChatCompletionResponse> {
        let messages = Self::build_chat_messages(system_message.clone(), history.clone());
        let request = self.create_request(messages)?;
        let response = self.client.chat().create(request).await?;
        Ok(response)
    }

    fn build_system_message(&self, question: &str) -> Result<ChatCompletionRequestSystemMessage> {
        let language = self.config.language.to_string();

        let system_prompt = format!(
            include_str!("../template/react_system_prompt.txt"),
            language = language,
            question = question,
            commands = Tools::description()?,
            response_format = RESPONSE_FORMAT,
        );

        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?;

        Ok(system_message)
    }

    fn create_request(
        &self,
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<CreateChatCompletionRequest> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.config.model.as_str())
            .temperature(self.config.temperature)
            .messages(messages)
            .build()?;

        Ok(request)
    }

    fn build_chat_messages(
        system_message: ChatCompletionRequestSystemMessage,
        history: Vec<ChatCompletionRequestMessage>,
    ) -> Vec<ChatCompletionRequestMessage> {
        let mut messages = vec![system_message.into()];
        messages.extend(history.iter().cloned());
        messages
    }
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
