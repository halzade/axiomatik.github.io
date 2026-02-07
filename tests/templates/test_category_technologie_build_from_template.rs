#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::application::technologie::technologie::TechnologieTemplate;
    use axiomatik_web::db::database_article_data::ShortArticleData;
    use axiomatik_web::trust::me::TrustError;

    #[test]
    fn test_technologie_build_from_template() -> Result<(), TrustError> {
        let articles_left = vec![ShortArticleData {
            article_file_name: "tech-1.html".to_string(),
            title: "Technology Article 1".to_string(),
            short_text: "Short text for technology 1".to_string(),
            image_288_path: "tech.jpg".to_string(),
            image_desc: "image_desc".to_string(),
        }];
        let articles_right = vec![];
        let template = TechnologieTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles_left: &articles_left,
            articles_right: &articles_right,
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Ze světa technologií"));
        assert!(rendered.contains("Technology Article 1"));
        assert!(rendered.contains("Short text for technology 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));

        Ok(())
    }
}
