use crate::tools::Tools;
use anyhow::{anyhow, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    },
    Client,
};
use tera::Tera;

pub(crate) struct Planning {
    system_temp: String,
    response_temp: String,
}

impl Planning {
    pub fn new() -> Self {
        Self {
            system_temp: include_str!("./template/react_system_prompt.txt").to_string(),
            response_temp: include_str!("./template/react_response_format.txt").to_string(),
        }
    }

    /// 将用户问题构建进系统消息
    ///
    /// 系统消息模版内容块说明：
    ///     头部内容：要求Agent独立解决问题，并且要严格遵循法律法规
    ///     GOAL: 需要解决的目标，即用户提出的问题
    ///     Constraints: 约束条件
    ///     Commands: 工具集，Agent可以使用的工具
    ///     Resources: 资源，Agent可以调用的资源
    ///     (重点)Performance Evaluation: 性能评估，包含反思、自我批评、思维链、子问题分解
    ///     Response Format: 响应格式，这里要求Agent返回json格式，方便反序列化
    pub fn build_system_message(
        &self,
        question: &str,
        language: &str,
    ) -> Result<ChatCompletionRequestSystemMessage> {
        // todo!: 可定义的人设说明
        let mut tera = Tera::default();
        tera.add_raw_template("system_prompt", &self.system_temp)?;

        let mut context = tera::Context::new();
        context.insert("language", language);
        context.insert("question", question);
        context.insert("commands", &Tools::to_string()?);
        context.insert("response_format", &self.response_temp);

        let system_prompt = tera.render("system_prompt", &context)?;

        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?;

        Ok(system_message)
    }

    pub fn build_command_result(&self, result: &str) -> Result<ChatCompletionRequestUserMessage> {
        // 提醒大模型结果必须是json格式
        let response_prompt = self
            .response_temp
            .lines()
            .next_back()
            .ok_or_else(|| anyhow!("No response format"))?;

        let content = format!("Command result: {}\n{}", result, response_prompt);

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(content)
            .build()?;

        Ok(user_message)
    }

    pub fn build_user_message(&self, content: &str) -> Result<ChatCompletionRequestUserMessage> {
        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(content)
            .build()?;

        Ok(user_message)
    }

    pub fn build_assistant_message(
        &self,
        content: &str,
    ) -> Result<ChatCompletionRequestAssistantMessage> {
        let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
            .content(content)
            .build()?;

        Ok(assistant_message)
    }

    pub async fn execute(
        &self,
        client: &Client<OpenAIConfig>,
        model: &str,
        temperature: f32,
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<CreateChatCompletionResponse> {
        let request = self.create_request(model, temperature, messages)?;
        // 大模型根据调用工具的返回结果，继续规划下一步
        let response = client.chat().create(request).await?;
        Ok(response)
    }

    fn create_request(
        &self,
        model: &str,
        temperature: f32,
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<CreateChatCompletionRequest> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .temperature(temperature)
            .messages(messages)
            .build()?;

        Ok(request)
    }
}
