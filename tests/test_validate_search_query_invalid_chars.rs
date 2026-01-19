#[cfg(test)]
mod tests {
    use axiomatik_web::validation::validate_search_query;

    #[test]
    fn test_validate_search_query_invalid_chars() {
        assert!(validate_search_query("test!").is_err());
        assert!(validate_search_query("search; drop table").is_err());
        assert!(validate_search_query("query <script>").is_err());
    }
}
