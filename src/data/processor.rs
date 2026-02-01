use crate::application::form::form_article_data_parser::ArticleData;
use std::fs;
use std::io::Error;
use std::path::Path;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("unknown category: {0}")]
    UnknownCategory(String),

    #[error("save web file io error: {0}")]
    SaveWebFileIOError(#[from] Error),
}

pub fn save_web_file(rendered_html: String, path: &str) -> Result<(), ProcessorError> {
    fs::write(Path::new("web").join(path), rendered_html)?;
    Ok(())
}

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
