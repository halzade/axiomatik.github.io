use std::str::FromStr;
use thiserror::Error;
use tracing::error;
use crate::application::article::form_article_data_parser::{ArticleData, ArticleError};


#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("Unknown category: {0}")]
    UnknownCategory(String),
}

// TODO try_from ??
pub fn process_category(raw_category: &str) -> Result<String, ProcessorError> {
    match raw_category {
        "zahranici" => Ok("zahraničí".into()),
        "republika" => Ok("republika".into()),
        "finance" => Ok("finance".into()),
        "technologie" => Ok("technologie".into()),
        "veda" => Ok("věda".into()),
        cat => {
            error!("Unknown category: {}", cat);
            Err(ProcessorError::UnknownCategory(cat.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CategoryEnum {
    Index,
    News,
    Finance,
    Republika,
    Technologie,
    Veda,
    Zahranici,
}

impl FromStr for CategoryEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "index" => Ok(CategoryEnum::Index),
            "news" => Ok(CategoryEnum::News),
            "finance" => Ok(CategoryEnum::Finance),
            "republika" => Ok(CategoryEnum::Republika),
            "technologie" => Ok(CategoryEnum::Technologie),
            "veda" => Ok(CategoryEnum::Veda),
            "zahranici" => Ok(CategoryEnum::Zahranici),
            _ => Err(()),
        }
    }
}

pub fn process_articles_create(article_data: ArticleData) -> Result<String, ArticleError> {
    
}

#[cfg(test)]
mod tests {
    use crate::data::processor::process_category;

    #[test]
    fn test_process_articles_create() {}

    #[test]
    fn test_process_category() {
        assert_eq!(process_category("zahranici").unwrap(), "zahraničí");
        assert_eq!(process_category("republika").unwrap(), "republika");
        assert_eq!(process_category("finance").unwrap(), "finance");
        assert_eq!(process_category("technologie").unwrap(), "technologie");
        assert_eq!(process_category("veda").unwrap(), "věda");
        assert!(process_category("invalid").is_err());
    }
}
