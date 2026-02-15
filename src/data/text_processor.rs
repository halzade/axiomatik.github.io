use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextProcessorError {
    #[error("Unknown category: {0}")]
    UnknownCategory(String),
}

pub fn process_short_text(raw_text: &str) -> String {
    raw_text
        .replace("\r\n", "\n")
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().replace("\n", "<br>\n"))
        .collect::<Vec<String>>()
        .join("</p><p>")
}

// TODO do this better
pub fn process_text(raw_text: &str) -> String {
    raw_text
        .replace("\r\n", "\n")
        .split("\n\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(|block| {
            let inner_html = block
                .split("\n\n")
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    if s.starts_with("   ") {
                        format!("<blockquote>{}</blockquote>", s.trim())
                    } else {
                        format!("<p>{}</p>", s.trim().replace("\n", " "))
                    }
                })
                .collect::<Vec<String>>()
                .join("");
            format!("<div class=\"container\">{}</div>", inner_html)
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_short_text() {
        let input = "Para 1\r\n\r\nPara 2\nLine 2";
        let output = process_short_text(input);
        assert_eq!(output, "Para 1</p><p>Para 2<br>\nLine 2");
    }

    #[test]
    fn test_process_text() {
        let input = "Block 1 Para 1\n\nBlock 1 Para 2\n\n\n   Block 2 Quote\n\nBlock 2 Para";
        let output = process_text(input);
        assert!(output.contains("<div class=\"container\">"));
        assert!(output.contains("<p>Block 1 Para 1</p>"));
        assert!(output.contains("<p>Block 1 Para 2</p>"));
        assert!(output.contains("<blockquote>Block 2 Quote</blockquote>"));
        assert!(output.contains("<p>Block 2 Para</p>"));
    }
}
