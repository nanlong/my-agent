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
        // 用户问题存入系统消息，作为Agent的任务目标
        let system_message = self.build_system_message(&question)?;
        let mut history: Vec<ChatCompletionRequestMessage> = Vec::new();

        let stream = stream! {
            // 并不发送给大模型，只是用来反馈给客户端
            let user_message = ChatCompletionRequestUserMessageArgs::default()
                .content(question)
                .build()?;

            yield Ok(user_message.into());

            'outer: for _step in 1..=self.config.max_steps {
                // 请求大模型
                let response = self.planning(system_message.clone(), history.clone()).await?;

                for choice in response.choices {
                    if let Some(assistant_prompt) = choice.message.content {
                        // 反序列化大模型返回的内容
                        let response: Response = serde_json::from_str(&assistant_prompt)?;

                        // 构建助手提示，放入历史消息，在下次对话中使用
                        let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
                            .content(assistant_prompt)
                            .build()?;

                        history.push(assistant_message.clone().into());
                        yield Ok(assistant_message.into());

                        // 如果大模型要求结束对话，说明任务完成了，可以退出
                        if response.command.name == "finish" {
                            break 'outer;
                        }

                        // 反序列化工具
                        let tool: Tools = response.command.try_into()?;
                        // 执行工具
                        let command_result = tool.execute().await?;
                        // 构建用户提示，将执行工具返回的结果存入，并放入历史消息，在下次对话中使用
                        let user_prompt = format!("Command result: {}\n{}", command_result, "Ensure that the response content conforms to the JSON Schema specification.");

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
        // 短期记忆：最后一条是上一次大模型要求调用工具的返回结果
        let messages = Self::build_chat_messages(system_message.clone(), history.clone());
        let request = self.create_request(messages)?;
        // 大模型根据调用工具的返回结果，继续规划下一步
        let response = self.client.chat().create(request).await?;
        Ok(response)
    }

    /// 将用户问题构建进系统消息
    ///
    /// 系统消息模版内容块说明：
    ///     头部内容：要求Agent独立解决问题，并且要严格遵循法律法规
    ///     GOAL: 需要解决的目标，即用户提出的问题
    ///     Constraints: 约束条件
    ///     Commands: 工具集，Agent可以使用的工具
    ///     Resources: 资源，Agent可以调用的资源
    ///     Performance Evaluation(重点): 性能评估，包含反思、自我批评、思维链、子问题分解
    ///     Response Format: 响应格式，这里要求Agent返回json格式，方便反序列化
    fn build_system_message(&self, question: &str) -> Result<ChatCompletionRequestSystemMessage> {
        // todo!: 可定义的人设说明
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
