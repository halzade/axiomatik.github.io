#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form::form_category::ZahraniciTemplate;
    use axiomatik_web::form::form_index::IndexCategoryArticleTemplate;
    use axiomatik_web::system::data_updates;

    #[test]
    fn test_zahranici_build_from_template() {
        data_updates::init_trivial_data();
        let template = ZahraniciTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles: vec![IndexCategoryArticleTemplate {
                url: "zahranici-1.html".to_string(),
                title: "Zahranici Article 1".to_string(),
                short_text: "Short text for zahranici 1".to_string(),
                is_first: true,
                image_path: "zahranici.jpg".to_string(),
                image_desc: "image_desc".to_string(),
                category_name: "Zahraničí".to_string(),
                category_url: "zahranici.html".to_string(),
            }],
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Ze zahraničí"));
        assert!(rendered.contains("Zahranici Article 1"));
        assert!(rendered.contains("Short text for zahranici 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));
    }
}
