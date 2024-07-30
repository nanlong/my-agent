use std::{
    fmt::{self, Display},
    str::FromStr,
};

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
    fn test_language_from_str() -> anyhow::Result<()> {
        assert_eq!("chinese".parse::<Language>()?, Language::Chinese);
        assert!("invalid".parse::<Language>().is_err());

        Ok(())
    }

    #[test]
    fn test_language_try_from() -> anyhow::Result<()> {
        assert_eq!(Language::try_from("chinese")?, Language::Chinese);
        assert!(Language::try_from("invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_language_display() {
        assert_eq!(format!("{}", Language::Chinese), "chinese");
    }
}
