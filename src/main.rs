use async_openai::types::ChatCompletionRequestMessage;
use futures::StreamExt;
use my_agent::agent::{ReActAgent, ReActAgentConfig, Response};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("MOONSHOT_API_KEY").expect("Missing MOONSHOT_API_KEY");
    let model = env::var("MOONSHOT_MODEL").expect("Missing MOONSHOT_MODEL");
    let api_base = env::var("MOONSHOT_API_BASE").expect("Missing MOONSHOT_API_BASE");

    let config = ReActAgentConfig::builder()
        .set_api_key(api_key)
        .set_model(model)
        .try_set_base_url(api_base.as_str())?
        .set_max_steps(10_usize)
        .build()?;

    let agent = ReActAgent::new(config);

    let question = "周杰伦今年多大了？他的年龄的0.23次方是多少？";

    let mut stream = agent.invoke(question).await?;

    while let Some(response) = stream.next().await {
        let response = response?;

        if let Some((role, content)) = match response {
            ChatCompletionRequestMessage::User(message) => {
                Some(("User", serde_json::to_string(&message.content)?))
            }
            ChatCompletionRequestMessage::Assistant(message) => {
                if let Some(content) = message.content {
                    let content = serde_json::from_str::<Response>(content.as_str())?;
                    let content = serde_json::to_string_pretty(&content)?;
                    Some(("Assistant", content))
                } else {
                    None
                }
            }
            _ => None,
        } {
            println!("{}: {}", role, content);
        }
    }

    Ok(())
}
