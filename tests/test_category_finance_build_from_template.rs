#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form::form_category::FinanceTemplate;
    use axiomatik_web::form::form_index::IndexCategoryArticleTemplate;
    use axiomatik_web::system::data_updates;

    #[test]
    fn test_finance_build_from_template() {
        data_updates::init_trivial_data();
        let template = FinanceTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles: vec![IndexCategoryArticleTemplate {
                url: "finance-1.html".to_string(),
                title: "Finance Article 1".to_string(),
                short_text: "Short text for finance 1".to_string(),
                is_first: true,
                image_path: "finance.jpg".to_string(),
                image_desc: "image_desc".to_string(),
                category_name: "Finance".to_string(),
                category_url: "finance.html".to_string(),
            }],
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Finance a kapitálové trhy"));
        assert!(rendered.contains("Ze světa financí"));
        assert!(rendered.contains("Finance Article 1"));
        assert!(rendered.contains("Short text for finance 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));
    }
}
