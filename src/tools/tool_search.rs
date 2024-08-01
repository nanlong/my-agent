use super::{
    search::tavily::{SearchParameters, Tavily},
    ToolExector, ToolPrompt,
};
use anyhow::{anyhow, Result};
use std::fmt::{self, Debug};

#[derive(Default)]
pub struct Search {
    client: Option<Tavily>,
    input: String,
}

impl Search {
    pub fn new(input: String) -> Self {
        let api_key = std::env::var("TAVILY_API_KEY").expect("Missing TAVILY_API_KEY");
        let client = Tavily::new(api_key);

        Self {
            client: Some(client),
            input,
        }
    }
}

impl ToolExector for Search {
    async fn execute(&self) -> Result<String> {
        let params = SearchParameters::builder().query(&self.input).build()?;

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("Client not initialized"))?;

        let response = client.search(params).await?;

        Ok(format!("{}", response))
    }
}

impl ToolPrompt for Search {
    fn command(&self) -> String {
        r#"{
            "name": "search",
            "description": "这是一个搜索引擎，当你已有的知识不足以完成目标任务时，可以通过它获取互联网上的信息",
            "args": [
                {
                    "name": "input",
                    "type": "string",
                    "description": "需要搜索的内容"
                }
            ]
        }"#.to_string()
    }

    fn resource(&self) -> String {
        "上网搜索和收集信息".to_string()
    }
}

impl Debug for Search {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Search")
            .field("input", &self.input)
            .finish()
    }
}
