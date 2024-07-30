use derive_builder::Builder;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
    str::FromStr,
};
use url::Url;

#[derive(Builder, Debug, Clone, PartialEq)]
#[builder(try_setter, setter(into, prefix = "set"))]
pub struct ReActAgentConfig {
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) model: String,
    #[builder(default = "Language::Chinese")]
    pub(crate) language: Language,
    #[builder(default = "10")]
    pub(crate) max_steps: usize,
}

impl ReActAgentConfig {
    pub fn builder() -> ReActAgentConfigBuilder {
        ReActAgentConfigBuilder::default()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum Language {
    #[default]
    Chinese,
    English,
}

impl FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "chinese" => Ok(Language::Chinese),
            "english" => Ok(Language::English),
            _ => Err(anyhow::anyhow!("Invalid language")),
        }
    }
}

impl<'a> TryFrom<&'a str> for Language {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::Chinese => write!(f, "chinese"),
            Language::English => write!(f, "english"),
        }
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
