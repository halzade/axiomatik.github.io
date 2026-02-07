#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::application::article::article::ArticleTemplate;
    use axiomatik_web::db::database_article_data::{MiniArticleData, ShortArticleData};
    use axiomatik_web::trust::utils::TrustError;

    #[test]
    fn test_article_build_from_template() -> Result<(), TrustError> {
        let related_articles = vec![
            ShortArticleData {
                article_file_name: "related-1.html".to_string(),
                title: "Related Article 1".to_string(),
                short_text: "Short text for related 1".to_string(),
                image_288_path: "img1.jpg".to_string(),
                image_desc: "image_desc".to_string(),
            },
            ShortArticleData {
                article_file_name: "related-2.html".to_string(),
                title: "Related Article 2".to_string(),
                short_text: "Short text for related 2".to_string(),
                image_288_path: "img2.jpg".to_string(),
                image_desc: "image_desc".to_string(),
            },
        ];

        let most_read = vec![MiniArticleData {
            article_file_name: "most-read-1.html".to_string(),
            image_50_path: "images/placeholder_50.png".to_string(),
            title: "Most Read 1".to_string(),
            mini_text: "Text for most read 1".to_string(),
            image_desc: "desc".to_string(),
        }];

        let template = ArticleTemplate {
            date: "Sobota 24. Ledna 2026".to_string(),
            weather: "-1°C | Praha".to_string(),
            name_day: "Milena".to_string(),

            author: "Lukáš ze Sametu".to_string(),

            title: "Jeden tisíc dnů".to_string(),
            text: "<p>Pouhých tisíc dnů nás dělí od vzestupu krajní pravice v Německu.</p>"
                .to_string(),
            image_path: "fasces-white.jpg".to_string(),
            image_desc: "Vlakové nádraží v ulici Baker Street".to_string(),
            video_path: Some("fasces-one.mp4".to_string()),
            audio_path: Some("audio.mp3".to_string()),
            category: "zahranici".to_string(),
            category_display: "Zahraničí".to_string(),
            related_articles,
            articles_most_read: most_read,
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Jeden tisíc dnů"));
        assert!(rendered.contains("Lukáš ze Sametu"));
        assert!(rendered.contains("Sobota 24. Ledna 2026"));
        assert!(rendered.contains("Pouhých tisíc dnů nás dělí"));
        assert!(rendered.contains("Related Article 1"));
        assert!(rendered.contains("Related Article 2"));
        assert!(rendered.contains("Most Read 1"));

        // Structure verification
        assert!(rendered.contains("<header class=\"w8 topbar\">"));
        assert!(rendered.contains("<nav class=\"w8 main-nav\">"));
        assert!(rendered.contains("<div class=\"w8 most\">"));
        assert!(rendered.contains("<section class=\"related-section\">"));

        Ok(())
    }
}
