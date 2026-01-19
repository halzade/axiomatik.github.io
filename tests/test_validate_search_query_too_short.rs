#[cfg(test)]
mod tests {
    use axiomatik_web::validation::validate_search_query;

    #[test]
    fn test_validate_search_query_too_short() {
        // TODO
        // validate_search_query doesn't check length anymore, handle_search does.
        // assert!(validate_search_query("").is_err());
        assert!(validate_search_query("ab").is_err());
        assert!(validate_search_query(".").is_err());
        assert!(validate_search_query("0").is_err());

        // TODO should be shorter
        assert!(validate_search_query("0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789X").is_err());
    }
}
