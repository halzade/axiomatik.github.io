#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form::form_category::RepublikaTemplate;
    use axiomatik_web::form::form_index::IndexCategoryArticleTemplate;
    use axiomatik_web::system::system_data;

    #[test]
    fn test_republika_build_from_template() {
        system_data::init_trivial_data();
        let template = RepublikaTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles: vec![IndexCategoryArticleTemplate {
                url: "rep-1.html".to_string(),
                title: "Republika Article 1".to_string(),
                short_text: "Short text for republica 1".to_string(),
                is_first: true,
                image_path: "rep.jpg".to_string(),
                image_description: "image_description".to_string(),
                category_name: "Republika".to_string(),
                category_url: "republika.html".to_string(),
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
