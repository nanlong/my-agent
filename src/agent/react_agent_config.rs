use super::Language;
use derive_builder::Builder;
use url::Url;

#[derive(Builder, Debug, Clone, PartialEq)]
#[builder(try_setter, setter(into, prefix = "set"))]
pub struct ReActAgentConfig {
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) model: String,
    // 返回的语言，默认中文
    #[builder(default = "Language::Chinese")]
    pub(crate) language: Language,
    // 最大调用轮数，在此之前未解决问题将会停止
    #[builder(default = "10")]
    pub(crate) max_steps: usize,
    #[builder(default = "0.3")]
    pub(crate) temperature: f32,
}

impl ReActAgentConfig {
    pub fn builder() -> ReActAgentConfigBuilder {
        ReActAgentConfigBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react_agent_config() -> anyhow::Result<()> {
        let config = ReActAgentConfig::builder()
            .set_api_key("my_api_key")
            .set_model("moonshot-v1-8k")
            .try_set_base_url("http://localhost")?
            .try_set_language("chinese")?
            .build()?;

        assert_eq!(config.api_key, "my_api_key");
        assert_eq!(config.model, "moonshot-v1-8k");
        assert_eq!(config.base_url, Url::parse("http://localhost")?);
        assert_eq!(config.language, Language::Chinese);
        assert_eq!(config.language.to_string(), "chinese");
        assert_eq!(config.max_steps, 10);

        Ok(())
    }
}
