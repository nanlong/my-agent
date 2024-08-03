use super::ReActAgentConfig;
use crate::{
    memory::ShortMemory,
    planning::Planning,
    tools::{ToolExector, Tools},
};
use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestToolMessageArgs,
    },
    Client,
};
use async_stream::stream;
use futures::Stream;
use std::pin::Pin;

type MessageStream = Pin<Box<dyn Stream<Item = Result<ChatCompletionRequestMessage>> + Send>>;

#[derive(Clone)]
pub struct ReActAgent {
    config: ReActAgentConfig,
    client: Client<OpenAIConfig>,
}

impl ReActAgent {
    pub fn new(config: ReActAgentConfig) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_key(config.api_key.as_str())
            .with_api_base(config.base_url.as_str());

        let client = Client::with_config(openai_config);

        Self { config, client }
    }

    pub async fn invoke(self, question: &str) -> Result<MessageStream> {
        let language = self.config.language.to_string();
        let planning = Planning::try_new()?;
        let mut short_memory = ShortMemory::new();

        short_memory.append(planning.build_system_message(question, &language)?.into());

        let user_message = planning.build_user_message(question)?;

        let stream = stream! {
            // 并不将第一条用户信息发送给大模型，只是用来反馈给客户端
            // 用户提出的问题已经存入系统消息，作为Agent的任务目标
            yield Ok(user_message.clone().into());

            'outer: for _step in 1..=self.config.max_steps {
                println!("history: {:#?}", short_memory.messages());

                // 请求大模型
                let response =
                    match planning.execute(&self.client, &self.config.model, self.config.temperature, short_memory.messages()).await {
                        Ok(response) => response,
                        Err(e) => {
                            println!("请求大模型遇到网络错误，马上进行重试操作... {:?}", e);
                            continue;
                        },
                    };

                println!("response: {:#?}", response);

                let response_message = response.choices.first().unwrap().message.clone();

                if let Some(tool_calls) = response_message.tool_calls {
                    // 构建调用工具的助手消息，放入短期记忆
                    let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
                        .tool_calls(tool_calls.clone())
                        .build()?;

                    short_memory.append(assistant_message.into());

                    // tool_calls 工具调用
                    for tool_call in tool_calls {
                        match Tools::try_from(tool_call.function.clone()) {
                            Ok(tool) => {
                                let result = tool.execute().await?;

                                // 将调用结果构建成工具消息，放入短期记忆
                                let tool_message = ChatCompletionRequestToolMessageArgs::default()
                                    .tool_call_id(tool_call.id)
                                    .content(result)
                                    .build()?;

                                short_memory.append(tool_message.clone().into());

                                yield Ok(tool_message.into());

                                // 如果工具是结束工具，则结束对话
                                if let Tools::Finish(_) = tool {
                                    break 'outer;
                                }
                            },
                            Err(e) => println!("工具序列化失败: \n{:?} \n{:?}", tool_call.function, e),
                        };
                    }
                }

                if let Some(assistant_prompt) = response_message.content {
                    if assistant_prompt.is_empty() {
                        // 如果助手提示为空，继续使用用户信息
                        short_memory.append(user_message.clone().into());
                        yield Ok(user_message.clone().into());
                    } else {
                        // 构建助手提示，放入短期记忆，在下次对话中使用
                        let assistant_message = planning.build_assistant_message(&assistant_prompt)?;
                        short_memory.append(assistant_message.clone().into());
                        yield Ok(assistant_message.into());
                    }
                }
            }
        };

        Ok(Box::pin(stream))
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
