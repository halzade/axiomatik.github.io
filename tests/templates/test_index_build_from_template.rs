#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::application::index::index::{
        IndexTemplate, MainArticleData, TopArticleData,
    };
    use axiomatik_web::db::database_article_data::ShortArticleData;
    use axiomatik_web::trust::utils::TrustError;

    #[test]
    fn test_index_build_from_template() -> Result<(), TrustError> {
        let template = IndexTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),

            articles_most_read: vec![],

            main_article: MainArticleData {
                article_file_name: "main-url".to_string(),
                title: "Main Title".to_string(),
                is_exclusive: false,
                short_text: "Main short text".to_string(),
                image_path: "img.jpg".to_string(),
                image_desc: "image_desc".to_string(),
            },
            second_article: TopArticleData {
                article_file_name: "second-url".to_string(),
                title: "Second Title".to_string(),
                short_text: "Second short text".to_string(),
            },
            third_article: TopArticleData {
                article_file_name: "third-url".to_string(),
                title: "Third Title".to_string(),
                short_text: "Third short text".to_string(),
            },
            z_republiky_articles: vec![ShortArticleData {
                article_file_name: "rep-1".to_string(),
                title: "Rep 1".to_string(),
                short_text: "Rep 1 text".to_string(),
                image_288_path: "rep1.jpg".to_string(),
                image_desc: "image_desc".to_string(),
            }],
            ze_zahranici_articles: vec![ShortArticleData {
                article_file_name: "for-1".to_string(),
                title: "For 1".to_string(),
                short_text: "For 1 text".to_string(),
                image_288_path: "for1.jpg".to_string(),
                image_desc: "image_desc".to_string(),
            }],
        };

        let rendered = template.render().expect("Failed to render template");
        std::fs::write("test-index.html", &rendered).expect("Failed to write test-index.html");

        let saved_content =
            std::fs::read_to_string("test-index.html").expect("Failed to read test-index.html");

        // Basic content verification
        assert!(saved_content.contains("Main Title"));
        assert!(saved_content.contains("Second Title"));
        assert!(saved_content.contains("Third Title"));
        assert!(saved_content.contains("Rep 1"));
        assert!(saved_content.contains("For 1"));

        // HTML structure verification (un-escaped)
        assert!(saved_content.contains("<section class=\"main-article\">"));

        // Cleanup
        assert!(std::fs::remove_file("test-index.html").is_ok());

        Ok(())
    }
}
