use super::{
    search::tavily::{SearchParameters, Tavily},
    ToolExector, ToolPrompt,
};
use anyhow::{anyhow, Result};
use async_openai::{
    error::OpenAIError,
    types::{
        ChatCompletionTool, ChatCompletionToolArgs, ChatCompletionToolType, FunctionCall,
        FunctionObjectArgs,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{self, Debug};

#[derive(Default)]
pub struct Search {
    client: Option<Tavily>,
    query: String,
}

impl Search {
    pub fn new(query: String) -> Self {
        let api_key = std::env::var("TAVILY_API_KEY").expect("Missing TAVILY_API_KEY");
        let client = Tavily::new(api_key);

        Self {
            client: Some(client),
            query,
        }
    }
}

impl ToolExector for Search {
    async fn execute(&self) -> Result<String> {
        let params = SearchParameters::builder().query(&self.query).build()?;

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
                    "name": "query",
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
            .field("query", &self.query)
            .finish()
    }
}

impl TryFrom<Search> for ChatCompletionTool {
    type Error = OpenAIError;

    fn try_from(_search: Search) -> Result<Self, Self::Error> {
        ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("search")
                    .description(r#"
                        通过搜索引擎搜索互联网上的内容。

                        当你的知识无法回答用户提出的问题，或用户请求你进行联网搜索时，调用此工具。请从与用户的对话中提取用户想要搜索的内容作为 query 参数的值。
                        搜索结果包含网站的标题、网站的地址（URL）以及网站简介。
                    "#)
                    .parameters(json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "用户搜索的内容，请从用户的提问或聊天上下文中提取。",
                            }
                        },
                        "required": ["query"],
                    }))
                    .build()?,
            )
            .build()
    }
}

#[derive(Serialize, Deserialize)]
struct SearchArgs {
    query: String,
}

impl TryFrom<FunctionCall> for Search {
    type Error = anyhow::Error;

    fn try_from(call: FunctionCall) -> Result<Self, Self::Error> {
        if call.name == "search" {
            let args: SearchArgs = serde_json::from_str(&call.arguments)?;
            Ok(Search::new(args.query))
        } else {
            Err(anyhow!("Invalid function call: {:?}", call))
        }
    }
}
