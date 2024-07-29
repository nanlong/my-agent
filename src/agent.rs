use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

#[allow(unused)]
static SUMMARY_PROMPT: &str = r#"You are an agent dedicated to summarising video transcripts.
You will receive a transcript and answer with main talking points of the video first,
followed by a complete summary of the transcript. Answer only in this format:


Talking points:
1. ..
2. ..
N. ..

Summary:
Summary of the transcript"#;

#[allow(unused)]
pub(crate) struct Agent {
    pub(crate) system: String,
    pub(crate) model: String,
    // 短期记忆
    pub(crate) history: Vec<Message>,
    pub(crate) client: Client<OpenAIConfig>,
}

#[allow(unused)]
impl Agent {
    async fn prompt(&mut self, input: impl Into<String>) -> Result<String, OpenAIError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(&self.system)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(input.into())
                    .build()?
                    .into(),
            ])
            .build()?;

        self.client.chat().create(request).await.map(|ret| {
            ret.choices[0]
                .message
                .content
                .clone()
                .unwrap_or("".to_string())
        })
    }
}

#[allow(unused)]
pub(crate) enum Role {
    Agent,
    User,
    System,
}

#[allow(unused)]
pub(crate) struct Message {
    pub(crate) content: String,
    pub(crate) role: Role,
}
