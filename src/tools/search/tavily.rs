use anyhow::Result;
use derive_builder::Builder;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct Tavily {
    api_key: String,
    base_url: String,
    client: Client,
}

impl Tavily {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.tavily.com".to_string(),
            client: Client::new(),
        }
    }

    pub async fn search(&self, params: SearchParameters) -> Result<SearchResponse> {
        let params = SearchParameters {
            api_key: self.api_key.clone(),
            ..params
        };

        let response = self
            .client
            .post(&format!("{}/search", self.base_url))
            .json(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Default, Builder, Serialize, Deserialize)]
#[builder(setter(into, strip_option), default)]
pub struct SearchParameters {
    query: String,
    api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_depth: Option<String>, // "basic" or "advanced"
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>, // general, news
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_answer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_raw_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_cache: Option<bool>,
}

impl SearchParameters {
    pub fn builder() -> SearchParametersBuilder {
        SearchParametersBuilder::default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub answer: Option<String>,
    pub query: String,
    pub response_time: f64,
    pub images: Vec<String>,
    pub results: Vec<SearchItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchItem {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_parameters() -> anyhow::Result<()> {
        let parameters = SearchParameters::builder()
            .query("周杰伦今年多大了？他的年龄的0.23次方是多少？")
            .api_key("my_api_key")
            .search_depth("basic")
            .topic("general")
            .max_results(5_usize)
            .include_images(true)
            .include_answer(true)
            .include_raw_content(true)
            .include_domains(vec!["baidu.com".to_string()])
            .exclude_domains(vec!["google.com".to_string()])
            .use_cache(true)
            .build()
            .unwrap();

        assert_eq!(
            parameters.query,
            "周杰伦今年多大了？他的年龄的0.23次方是多少？"
        );
        assert_eq!(parameters.api_key, "my_api_key");
        assert_eq!(parameters.search_depth, Some("basic".to_string()));
        assert_eq!(parameters.topic, Some("general".to_string()));
        assert_eq!(parameters.max_results, Some(5));
        assert_eq!(parameters.include_images, Some(true));
        assert_eq!(parameters.include_answer, Some(true));
        assert_eq!(parameters.include_raw_content, Some(true));
        assert_eq!(
            parameters.include_domains,
            Some(vec!["baidu.com".to_string()])
        );
        assert_eq!(
            parameters.exclude_domains,
            Some(vec!["google.com".to_string()])
        );
        assert_eq!(parameters.use_cache, Some(true));

        let parameters = SearchParameters::builder()
            .query("周杰伦今年多大了？他的年龄的0.23次方是多少？")
            .api_key("my_api_key")
            .build()?;

        let params = serde_json::to_string(&parameters)?;

        assert_eq!(
            params,
            r#"{"query":"周杰伦今年多大了？他的年龄的0.23次方是多少？","api_key":"my_api_key"}"#
        );

        Ok(())
    }
}
