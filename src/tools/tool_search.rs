use super::{
    search::tavily::{SearchParameters, Tavily},
    ToolExecute,
};
use anyhow::{anyhow, Result};
use std::fmt::{self, Debug, Display};

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

impl ToolExecute for Search {
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

impl Debug for Search {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Search")
            .field("input", &self.input)
            .finish()
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = r#"Search : "search", args: "input": "<search content>""#;
        write!(f, "{}", description)
    }
}
