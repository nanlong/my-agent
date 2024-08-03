use async_openai::types::ChatCompletionRequestMessage;
use chrono::prelude::*;
use futures::StreamExt;
use my_agent::agent::{ReActAgent, ReActAgentConfig};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    let model = env::var("OPENAI_MODEL").expect("Missing OPENAI_MODEL");
    let api_base = env::var("OPENAI_API_BASE").expect("Missing OPENAI_API_BASE");

    let config = ReActAgentConfig::builder()
        .set_api_key(api_key)
        .set_model(model)
        .try_set_base_url(api_base.as_str())?
        .set_max_steps(10_usize)
        .build()?;

    let agent = ReActAgent::new(config);

    // let question = "周杰伦今年多大了？他的年龄的0.23次方是多少？";
    // let question = "制作一份关于周杰伦的简历";
    let question = "请联网搜索 Context Caching，并告诉我它是什么。";

    let mut stream = agent.invoke(question).await?;

    while let Some(response) = stream.next().await {
        let response = response?;

        if let Some((role, content)) = match response {
            ChatCompletionRequestMessage::User(message) => {
                Some(("User", serde_json::to_string(&message.content)?))
            }
            ChatCompletionRequestMessage::Assistant(message) => {
                message.content.map(|content| ("Assistant", content))
            }
            ChatCompletionRequestMessage::Tool(message) => Some((
                "Tool",
                format!("{} - {}", message.tool_call_id, message.content),
            )),
            _ => None,
        } {
            let local = Local::now().format("%m-%d %H:%M:%S").to_string();
            println!("[{}] {}: {}", local, role, content);
        }
    }

    Ok(())
}
