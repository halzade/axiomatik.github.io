pub fn validate_input(input: &str) -> Result<(), &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        assert!(validate_input("").is_ok());
        assert!(validate_input("Hello\nWorld\r\t").is_ok());
        assert!(validate_input("Příliš žluťoučký kůň úpěl ďábelské ódy").is_ok()); // Non-ASCII UTF-8 is allowed
        assert!(validate_input("Hello \x01 World").is_err()); // ASCII control character
        assert!(validate_input("Hello \x7F World").is_err()); // ASCII DEL
    }

    #[test]
    fn test_validate_search_query() {
        assert!(validate_search_query("").is_ok());
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
