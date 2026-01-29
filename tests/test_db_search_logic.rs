#[cfg(test)]
mod tests {
    use axiomatik_web::db::database_article::Article;
    use axiomatik_web::db::database_article;
    use axiomatik_web::test_framework::script_base;

    #[tokio::test]
    async fn test_db_search_logic() {
        script_base::setup_before_tests_once().await;

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

            // TODO
            related_articles: vec![],
            is_main: false,
            is_exclusive: false,
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
            related_articles: vec![],
            is_main: false,
            is_exclusive: false,
            views: 0,
        };

        database_article::create_article(article1).await.unwrap();
        database_article::create_article(article2).await.unwrap();

        let articles_o = database_article::articles_by_words("match").await;

        match articles_o {
            None => {
                panic!("Test: no Articles found");
            }
            Some(articles) => {
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

                // TODO ?
                assert_eq!(results.len(), 2);
                assert_eq!(results[0].0, 2);
                assert_eq!(results[0].1, "article1");
                assert_eq!(results[1].0, 2);
                assert_eq!(results[1].1, "article2");
            }
        }
    }
}
