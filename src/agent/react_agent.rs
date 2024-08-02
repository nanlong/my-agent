use super::{response::Response, ReActAgentConfig};
use crate::{
    memory::ShortMemory,
    planning::Planning,
    tools::{ToolExector, Tools},
};
use anyhow::Result;
use async_openai::{config::OpenAIConfig, types::ChatCompletionRequestMessage, Client};
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
            yield Ok(user_message.into());

            'outer: for _step in 1..=self.config.max_steps {
                // 请求大模型
                let response =
                    match planning.execute(&self.client, &self.config.model, self.config.temperature, short_memory.messages()).await {
                        Ok(response) => response,
                        Err(_) => {
                            println!("Failed to get response from OpenAI, retrying...");
                            continue;
                        },
                    };

                for choice in response.choices {
                    if let Some(assistant_prompt) = choice.message.content {
                        // 反序列化大模型返回的内容
                        let response  = match serde_json::from_str::<Response>(&assistant_prompt) {
                            Ok(response) => response,
                            Err(_) => {
                                // 如果不能正常解析，修复json格式
                                let user_message = planning.build_fixjson_message(&assistant_prompt)?;
                                short_memory.append(user_message.clone().into());
                                continue;
                            },
                        };

                        // 构建助手提示，放入短期记忆，在下次对话中使用
                        let assistant_message = planning.build_assistant_message(&assistant_prompt)?;
                        // println!("Assistant Debug: {:?}", assistant_message);
                        short_memory.append(assistant_message.into());

                        // 用户可以看到的
                        let assistant_message = planning.build_assistant_message(&response.thoughts.speak)?;
                        yield Ok(assistant_message.into());

                        // 反序列化工具
                        let tool = Tools::try_from(response.command)?;

                        // 执行工具
                        let command_result = tool.execute().await?;

                        // 如果大模型要求结束对话，说明任务完成了，可以退出
                        if let Tools::Finish(_) = tool {
                            let assistant_message = planning.build_assistant_message(&command_result)?;
                            yield Ok(assistant_message.into());

                            break 'outer;
                        }

                        // 构建用户提示，将执行工具返回的结果存入，并放入短期记忆，在下次对话中使用
                        let user_message = planning.build_command_result(&command_result)?;
                        short_memory.append(user_message.clone().into());
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
