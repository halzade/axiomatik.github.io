#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form_category::VedaTemplate;
    use axiomatik_web::form_index::IndexCategoryArticleTemplate;

    #[test]
    fn test_veda_build_from_template() {
        axiomatik_web::data::init_trivial();
        let template = VedaTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles: vec![IndexCategoryArticleTemplate {
                url: "veda-1.html".to_string(),
                title: "Veda Article 1".to_string(),
                short_text: "Short text for veda 1".to_string(),
                is_first: true,
                image_path: "veda.jpg".to_string(),
                image_description: "image_description".to_string(),
                category_name: "Věda".to_string(),
                category_url: "veda.html".to_string(),
            }],
        };

        let rendered = template.render().expect("Failed to render template");
        
        // Basic content verification
        assert!(rendered.contains("Věda a výzkum"));
        assert!(rendered.contains("Veda Article 1"));
        assert!(rendered.contains("Short text for veda 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));
    }
}
