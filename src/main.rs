use futures::StreamExt;
use my_agent::agent::{ReActAgent, ReActAgentConfig};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let api_key = env::var("MOONSHOT_API_KEY")?;
    let model = env::var("MOONSHOT_MODEL")?;
    let api_base = env::var("MOONSHOT_API_BASE")?;

    let config = ReActAgentConfig::builder()
        .set_api_key(api_key)
        .set_model(model)
        .try_set_base_url(api_base.as_str())?
        .set_max_steps(3_usize)
        .build()?;

    let agent = ReActAgent::new(config);

    let question = "周杰伦今年多大了？他的年龄的0.23次方是多少？";

    let mut stream = agent.invoke(question).await?;

    while let Some(Ok(response)) = stream.next().await {
        let content = response.content.as_ref();
        println!("Content: {}", content.unwrap());

        let response: Value = serde_json::from_str(content.unwrap())?;
        println!(
            "Agent spake: {}",
            &response.get("thoughts").unwrap().get("speak").unwrap()
        );
    }

    Ok(())
}
