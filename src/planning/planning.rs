use crate::tools::Tools;
use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageArgs, ChatCompletionToolChoiceOption,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    },
    Client,
};
use tera::{Context, Tera};

pub(crate) struct Planning {
    engine: Tera,
}

impl Planning {
    pub fn try_new() -> Result<Self> {
        let engine = Tera::new("templates/**/*")?;
        Ok(Planning { engine })
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
        let response_format = self
            .engine
            .render("response_format.prompt", &Default::default())?;

        let mut context = Context::new();
        context.insert("language", language);
        context.insert("question", question);
        context.insert("response_format", &response_format);

        let system_prompt = self.engine.render("system.prompt", &context)?;

        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?;

        Ok(system_message)
    }

    pub fn _build_command_result(&self, result: &str) -> Result<ChatCompletionRequestUserMessage> {
        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(result)
            .build()?;

        Ok(user_message)
    }

    pub fn _build_fixjson_message(
        &self,
        content: &str,
    ) -> Result<ChatCompletionRequestUserMessage> {
        let mut context = Context::new();
        context.insert("response", content);

        let content = self.engine.render("fix_response_format.prompt", &context)?;

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
            .tools(Tools::list())
            // 这里应设置为Required，强制大模型每次都调用工具，但是某些大模型不支持此选项
            .tool_choice(ChatCompletionToolChoiceOption::Auto)
            // .response_format(ChatCompletionResponseFormat {
            //     r#type: ChatCompletionResponseFormatType::JsonObject,
            // })
            .build()?;

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_template() -> Result<()> {
        let planning = Planning::try_new()?;

        assert!(planning.engine.get_template("system.prompt").is_ok());
        Ok(())
    }
}
