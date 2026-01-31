#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form::form_category::TechnologieTemplate;
    use axiomatik_web::form::form_index::IndexCategoryArticleTemplate;
    use axiomatik_web::system::data_updates;

    #[test]
    fn test_technologie_build_from_template() {
        data_updates::init_trivial_data();
        let template = TechnologieTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles: vec![IndexCategoryArticleTemplate {
                url: "tech-1.html".to_string(),
                title: "Technology Article 1".to_string(),
                short_text: "Short text for technology 1".to_string(),
                is_first: true,
                image_path: "tech.jpg".to_string(),
                image_description: "image_description".to_string(),
                category_name: "Technologie".to_string(),
                category_url: "technologie.html".to_string(),
            }],
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Ze světa technologií"));
        assert!(rendered.contains("Technology Article 1"));
        assert!(rendered.contains("Short text for technology 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));
    }
}
