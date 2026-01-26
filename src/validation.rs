// TODO validate if not too long
// TODO I don't like the Error return type
pub fn validate_required_string(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126 || c == '\n' || c == '\r' || c == '\t') {
                return Err("Invalid character detected");
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }

    if validate_required(&input) {
        // validated
        return Ok(());
    }
    // required but empty
    Err("is required but not set")
}

// TODO
pub fn validate_required_text(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126) {
                return Err("Invalid character detected");
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }

    if validate_required(&input) {
        // validated
        return Ok(());
    }
    // required but empty
    Err("is required but not set")
}

// TODO Unit tests
pub fn validate_optional_string(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126 || c == '\n' || c == '\r' || c == '\t') {
                return Err("Invalid character detected");
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }
    Ok(())
}

pub fn validate_search_query(input: &str) -> Result<(), &'static str> {
    if (input.len() < 3) || (input.len() > 100) {
        return Err("Input to short or too long");
    }
    for c in input.chars() {
        if c.is_ascii() {
            // No system characters (0-31, 127) and no special characters
            // Allow only alphanumeric and spaces for search
            if !c.is_ascii_alphanumeric() && c != ' ' {
                return Err("Only alphanumeric characters and spaces are allowed in search");
            }
        } else if !c.is_alphanumeric() {
            return Err("Only alphanumeric characters are allowed in search");
        }
    }
    Ok(())
}

pub fn validate_input_simple(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if !c.is_ascii_alphanumeric() {
            if c != '_' {
                return Err("Incorrect character detected");
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
        assert!(validate_required_string("").is_ok());
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
}
