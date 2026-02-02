#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::application::zahranici::zahranici::ZahraniciTemplate;
    use axiomatik_web::db::database_article_data::ShortArticleData;

    #[test]
    fn test_zahranici_build_from_template() {
        let articles_left = vec![ShortArticleData {
            url: "zahranici-1.html".to_string(),
            title: "Zahranici Article 1".to_string(),
            short_text: "Short text for zahranici 1".to_string(),
            image_288_path: "zahranici.jpg".to_string(),
            image_desc: "image_desc".to_string(),
        }];
        let articles_right = vec![];
        let template = ZahraniciTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles_left: &articles_left,
            articles_right: &articles_right,
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Ze zahraničí"));
        assert!(rendered.contains("Zahranici Article 1"));
        assert!(rendered.contains("Short text for zahranici 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));
    }
}
