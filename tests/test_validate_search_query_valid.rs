#[cfg(test)]
mod tests {
    use axiomatik_web::validation::validate_search_query;

    #[test]
    fn test_validate_search_query_valid() {
        assert!(validate_search_query("test query").is_ok());
        assert!(validate_search_query("123 search").is_ok());
        assert!(validate_search_query("český dotaz").is_ok());
    }
}
