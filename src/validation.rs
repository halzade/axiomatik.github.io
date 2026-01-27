use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid character detected")]
    InvalidCharacter,
    #[error("Input too short or too long")]
    InvalidLength,
    #[error("Only alphanumeric characters and spaces are allowed in search")]
    SearchOnlyAlphanumericAndSpaces,
    #[error("Only alphanumeric characters are allowed in search")]
    SearchOnlyAlphanumeric,
    #[error("Incorrect character detected")]
    SimpleInputIncorrectCharacter,
    #[error("is required but not set")]
    RequiredFieldMissing,
}

// TODO validate if not too long
pub fn validate_required_string(input: &str) -> Result<(), ValidationError> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126 || c == '\n' || c == '\r' || c == '\t') {
                return Err(ValidationError::InvalidCharacter);
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }

    if validate_required(&input) {
        // validated
        return Ok(());
    }
    // required but empty
    Err(ValidationError::RequiredFieldMissing)
}

// TODO
pub fn validate_required_text(input: &str) -> Result<(), ValidationError> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126) {
                return Err(ValidationError::InvalidCharacter);
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }

    if validate_required(&input) {
        // validated
        return Ok(());
    }
    // required but empty
    Err(ValidationError::RequiredFieldMissing)
}

// TODO Unit tests
pub fn validate_optional_string(input: &str) -> Result<(), ValidationError> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126 || c == '\n' || c == '\r' || c == '\t') {
                return Err(ValidationError::InvalidCharacter);
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }
    Ok(())
}

pub fn validate_search_query(input: &str) -> Result<(), ValidationError> {
    if (input.len() < 3) || (input.len() > 40) {
        return Err(ValidationError::InvalidLength);
    }
    for c in input.chars() {
        if c.is_ascii() {
            // No system characters (0-31, 127) and no special characters
            // Allow only alphanumeric and spaces for search
            if !c.is_ascii_alphanumeric() && c != ' ' {
                return Err(ValidationError::SearchOnlyAlphanumericAndSpaces);
            }
        } else if !c.is_alphanumeric() {
            return Err(ValidationError::SearchOnlyAlphanumeric);
        }
    }
    Ok(())
}

pub fn validate_input_simple(input: &str) -> Result<(), ValidationError> {
    for c in input.chars() {
        if !c.is_ascii_alphanumeric() {
            if c != '_' {
                return Err(ValidationError::SimpleInputIncorrectCharacter);
            }
        }
    }
    Ok(())
}

fn validate_required(input: &str) -> bool {
    !input.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_required_string() {
        assert!(validate_required_string("").is_err());
        assert!(validate_required_string("Hello\nWorld\r\t").is_ok());
        assert!(validate_required_string("Příliš žluťoučký kůň úpěl ďábelské ódy").is_ok()); // Non-ASCII UTF-8 is allowed
        assert!(validate_required_string("Hello \x01 World").is_err()); // ASCII control character
        assert!(validate_required_string("Hello \x7F World").is_err()); // ASCII DEL
    }

    #[test]
    fn test_validate_search_query() {
        assert!(validate_search_query("").is_err());
        assert!(validate_search_query("Hello World").is_ok());
        assert!(validate_search_query("Hello123").is_ok());
        assert!(validate_search_query("Příliš").is_ok()); // Non-ASCII alphanumeric is allowed
        assert!(validate_search_query("Hello!").is_err()); // Special character
        assert!(validate_search_query("Hello\nWorld").is_err()); // Whitespace other than space
    }

    #[test]
    fn test_validate_input_simple() {
        assert!(validate_input_simple("").is_ok());
        assert!(validate_input_simple("Hello_World123").is_ok());
        assert!(validate_input_simple("Hello World").is_err()); // Space is not allowed
        assert!(validate_input_simple("Příliš").is_err()); // Non-ASCII is not allowed
        assert!(validate_input_simple("Hello-World").is_err()); // Hyphen is not allowed
    }

    #[test]
    fn test_validate_search_query_too_short() {
        assert!(validate_search_query("").is_err());
        assert!(validate_search_query("ab").is_err());
        assert!(validate_search_query(".").is_err());
        assert!(validate_search_query("0").is_err());
        assert!(validate_search_query("1234567890123456789012345678901234567890").is_ok());
        assert!(validate_search_query("12345678901234567890123456789012345678901").is_err());
    }

    #[test]
    fn test_validate_search_query_invalid_chars() {
        assert!(validate_search_query("test!").is_err());
        assert!(validate_search_query("search; drop table").is_err());
        assert!(validate_search_query("query <script>").is_err());
    }

    #[test]
    fn test_validate_search_query_valid() {
        assert!(validate_search_query("test query").is_ok());
        assert!(validate_search_query("123 search").is_ok());
        assert!(validate_search_query("český dotaz").is_ok());
    }
}
