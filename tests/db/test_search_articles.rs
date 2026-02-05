#[cfg(test)]
mod tests {
    use axiomatik_web::db::database_article;
    use axiomatik_web::db::database_article_data::Article;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::TrustError;
    use chrono::Utc;
    use surrealdb_types::Uuid;

    #[tokio::test]
    async fn test_db_search_logic() -> Result<(), TrustError> {
        script_base::setup_before_tests_once().await;

        let now = Utc::now();

        let article1 = Article {
            uuid: Uuid::new(),
            author: "author".to_string(),
            user: "user".to_string(),
            date: now,
            date_str: "date".to_string(),
            title: "Title 1".to_string(),
            text: "One word match here.".to_string(),
            short_text: "short".to_string(),
            mini_text: "mini".to_string(),
            file_base: "article1".to_string(),
            image_desc: "desc".to_string(),
            image_50_path: "img50".to_string(),
            image_288_path: "img288".to_string(),
            image_440_path: "img440".to_string(),
            image_820_path: "img820".to_string(),
            has_video: false,
            video_path: "".to_string(),
            has_audio: false,
            audio_path: "".to_string(),
            category: "cat".to_string(),
            related_articles: vec![],
            is_main: false,
            is_exclusive: false,
            views: 0,
        };

        let article2 = Article {
            uuid: Uuid::new(),
            author: "author".to_string(),
            user: "user".to_string(),
            date: now,
            date_str: "date".to_string(),
            title: "Title 2".to_string(),
            text: "Two words match here match.".to_string(),
            short_text: "short".to_string(),
            mini_text: "mini".to_string(),
            file_base: "article2".to_string(),
            image_desc: "desc".to_string(),
            image_50_path: "img50".to_string(),
            image_288_path: "img288".to_string(),
            image_440_path: "img440".to_string(),
            image_820_path: "img820".to_string(),
            has_video: false,
            video_path: "".to_string(),
            has_audio: false,
            audio_path: "".to_string(),
            category: "cat".to_string(),
            related_articles: vec![],
            is_main: false,
            is_exclusive: false,
            views: 0,
        };

        database_article::create_article(article1).await;
        database_article::create_article(article2).await;

        let articles = database_article::articles_by_words(vec!["match".into()], 3)
            .await
            .unwrap();

        assert_eq!(articles.len(), 2);
        // articles_by_words returns Vec<ShortArticleData> which has url, title, short_text, image_288_path, image_desc
        // In database_article.rs it seems it selects * from an article which might be mapped to ShortArticleData
        // Let's check how they are ordered. Ordered by date DESC.

        let titles: Vec<String> = articles.iter().map(|a| a.title.clone()).collect();
        assert!(titles.contains(&"Title 1".to_string()));
        assert!(titles.contains(&"Title 2".to_string()));

        Ok(())
    }
}
