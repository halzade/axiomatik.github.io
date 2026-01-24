#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form_new_article::ArticleTemplate;
    use axiomatik_web::form_index::{IndexCategoryArticleTemplate, IndexArticleMostRead};

    #[test]
    fn test_article_build_from_template() {
        axiomatik_web::data::init_trivial();
        
        let related_articles = vec![
            IndexCategoryArticleTemplate {
                url: "related-1.html".to_string(),
                title: "Related Article 1".to_string(),
                short_text: "Short text for related 1".to_string(),
                is_first: false,
                image_path: "img1.jpg".to_string(),
                category_name: "Republika".to_string(),
                category_url: "republika.html".to_string(),
            },
            IndexCategoryArticleTemplate {
                url: "related-2.html".to_string(),
                title: "Related Article 2".to_string(),
                short_text: "Short text for related 2".to_string(),
                is_first: false,
                image_path: "img2.jpg".to_string(),
                category_name: "Zahraničí".to_string(),
                category_url: "zahranici.html".to_string(),
            },
        ];

        let most_read = vec![
            IndexArticleMostRead {
                image_url_50: "images/placeholder_50.png".to_string(),
                title: "Most Read 1".to_string(),
                text: "Text for most read 1".to_string(),
            }
        ];

        let template = ArticleTemplate {
            title: "Jeden tisíc dnů".to_string(),
            author: "Lukáš ze Sametu".to_string(),
            date: "Sobota 24. Ledna 2026".to_string(),
            text: "<p>Pouhých tisíc dnů nás dělí od vzestupu krajní pravice v Německu.</p>".to_string(),
            image_path: "fasces-white.jpg".to_string(),
            image_description: "Vlakové nádraží v ulici Baker Street".to_string(),
            video_path: Some("fasces-one.mp4".to_string()),
            audio_path: Some("audio.mp3".to_string()),
            category: "zahranici".to_string(),
            category_display: "Zahraničí".to_string(),
            related_articles,
            weather: "-1°C | Praha".to_string(),
            name_day: "Milena".to_string(),
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
    }
}
