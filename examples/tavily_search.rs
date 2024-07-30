use my_agent::tools::search::tavily::{SearchParameters, Tavily};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let api_key = env::var("TAVILY_API_KEY")?;
    let tavily = Tavily::new(api_key);

    let params = SearchParameters::builder()
        .query("周杰伦今年多大了？他的年龄的0.23次方是多少？")
        .build()?;

    let response = tavily.search(params).await?;

    println!("Response: {:?}", response.results.len());

    Ok(())
}
