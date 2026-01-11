use axiomatik_web::validate_search_query;

#[test]
fn test_validate_search_query_too_short() {
    assert!(validate_search_query("").is_err());
    assert!(validate_search_query("ab").is_err());
    assert!(validate_search_query(".").is_err());
    assert!(validate_search_query("0").is_err());
    assert!(validate_search_query("0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789X").is_err());
}

#[test]
fn test_validate_search_query_valid() {
    assert!(validate_search_query("test query").is_ok());
    assert!(validate_search_query("123 search").is_ok());
    assert!(validate_search_query("český dotaz").is_ok());
}

#[test]
fn test_validate_search_query_invalid_chars() {
    assert!(validate_search_query("test!").is_err());
    assert!(validate_search_query("search; drop table").is_err());
    assert!(validate_search_query("query <script>").is_err());
}

#[test]
fn test_search_ranking_logic() {
    let snippets = vec![
        "One word match here.",
        "Two words match here here.",
        "No match at all.",
    ];
    let query = "word match";
    let search_words: Vec<&str> = query.split_whitespace().collect();

    let mut results = Vec::new();
    for content in snippets {
        let mut match_count = 0;
        for word in &search_words {
            if content.to_lowercase().contains(&word.to_lowercase()) {
                match_count += 1;
            }
        }
        if match_count > 0 {
            results.push((match_count, content));
        }
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 2);
    assert_eq!(results[1].0, 2);
    assert_eq!(results[2].0, 1); // "No match at all." matches "match"
}

#[test]
fn test_search_ranking_logic_different_counts() {
    let snippets = vec!["Only word.", "Both word and match.", "Neither."];
    let query = "word match";
    let search_words: Vec<&str> = query.split_whitespace().collect();

    let mut results = Vec::new();
    for content in snippets {
        let mut match_count = 0;
        for word in &search_words {
            if content.to_lowercase().contains(&word.to_lowercase()) {
                match_count += 1;
            }
        }
        if match_count > 0 {
            results.push((match_count, content));
        }
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, 2);
    assert!(results[0].1.contains("Both"));
    assert_eq!(results[1].0, 1);
    assert!(results[1].1.contains("Only"));
}
