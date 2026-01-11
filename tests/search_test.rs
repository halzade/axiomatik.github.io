use axiomatik_web::validate_search_query;
use axiomatik_web::db::{Article, Database};

#[test]
fn test_validate_search_query_too_short() {
    // TODO
    // validate_search_query doesn't check length anymore, handle_search does.
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

#[tokio::test]
async fn test_db_search_logic() {
    let db = Database::new("mem://").await;
    
    let article1 = Article {
        author: "author".to_string(),
        created_by: "user".to_string(),
        date: "date".to_string(),
        title: "Title 1".to_string(),
        text: "One word match here.".to_string(),
        short_text: "short".to_string(),
        article_file_name: "article1".to_string(),
        image_url: "img".to_string(),
        image_description: "desc".to_string(),
        video_url: None,
        audio_url: None,
        category: "cat".to_string(),
        related_articles: "".to_string(),
        views: 0,
    };
    
    let article2 = Article {
        author: "author".to_string(),
        created_by: "user".to_string(),
        date: "date".to_string(),
        title: "Title 2".to_string(),
        text: "Two words match here match.".to_string(),
        short_text: "short".to_string(),
        article_file_name: "article2".to_string(),
        image_url: "img".to_string(),
        image_description: "desc".to_string(),
        video_url: None,
        audio_url: None,
        category: "cat".to_string(),
        related_articles: "".to_string(),
        views: 0,
    };

    db.create_article(article1).await.unwrap();
    db.create_article(article2).await.unwrap();

    let articles = db.get_all_articles().await.unwrap();
    let query = "word match";
    let search_words: Vec<&str> = query.split_whitespace().collect();

    let mut results = Vec::new();
    for article in articles {
        let mut match_count = 0;
        let text_lower = article.text.to_lowercase();
        for word in &search_words {
            if text_lower.contains(&word.to_lowercase()) {
                match_count += 1;
            }
        }
        if match_count > 0 {
            results.push((match_count, article.article_file_name));
        }
    }

    results.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, 2);
    assert_eq!(results[0].1, "article1");
    assert_eq!(results[1].0, 2); 
    assert_eq!(results[1].1, "article2");
}
