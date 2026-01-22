#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form_category::RepublikaTemplate;
    use axiomatik_web::form_index::IndexCategoryArticleTemplate;

    #[test]
    fn test_republika_build_from_template() {
        let template = RepublikaTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles: vec![IndexCategoryArticleTemplate {
                url: "rep-1.html".to_string(),
                title: "Republika Article 1".to_string(),
                short_text: "Short text for republica 1".to_string(),
            }],
        };

        let rendered = template.render().expect("Failed to render template");
        
        // Basic content verification
        assert!(rendered.contains("Republika"));
        assert!(rendered.contains("Z naší republiky"));
        assert!(rendered.contains("Republika Article 1"));
        assert!(rendered.contains("Short text for republica 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));
    }
}
