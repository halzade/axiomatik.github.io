#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::application::news::news::NewsTemplate;
    use axiomatik_web::db::database_article_data::ShortArticleData;

    #[test]
    fn test_news_build_from_template() {
        let template = NewsTemplate {
            date: "Saturday, January 24, 2026".to_string(),
            weather: "-1°C | Prague".to_string(),
            name_day: "Milena".to_string(),
            articles_most_read: vec![],
            z_republiky: vec![ShortArticleData {
                url: "news-1.html".to_string(),
                title: "Republika News 1".to_string(),
                short_text: "Short text for republica".to_string(),
                image_288_path: "img.jpg".to_string(),
                image_desc: "image_desc".to_string(),
            }],
            ze_zahranici: vec![],
            technologie: vec![],
            veda: vec![],
            finance: vec![],
        };

        let rendered = template.render().expect("Failed to render template");
        
        // Basic content verification
        assert!(rendered.contains("NEXO.cz — Zprávy"));
        assert!(rendered.contains("Republika News 1"));
        assert!(rendered.contains("Z naší republiky"));
        assert!(rendered.contains("Ze světa technologií"));
        assert!(rendered.contains("Věda a výzkum"));
        assert!(rendered.contains("Ze zahraničí"));
        assert!(rendered.contains("Ze světa financí"));
        assert!(rendered.contains("Saturday, January 24, 2026"));
    }
}
