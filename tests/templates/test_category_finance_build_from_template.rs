#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::application::finance::finance::FinanceTemplate;
    use axiomatik_web::db::database_article_data::ShortArticleData;
    use axiomatik_web::trust::me::TrustError;

    #[test]
    fn test_finance_build_from_template() -> Result<(), TrustError> {
        let articles_left = vec![ShortArticleData {
            article_file_name: "finance-1.html".to_string(),
            title: "Finance Article 1".to_string(),
            short_text: "Short text for finance 1".to_string(),
            image_288_path: "finance.jpg".to_string(),
            image_desc: "image_desc".to_string(),
        }];
        let articles_right = vec![];
        let template = FinanceTemplate {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            articles_most_read: vec![],
            articles_left: &articles_left,
            articles_right: &articles_right,
        };

        let rendered = template.render().expect("Failed to render template");

        // Basic content verification
        assert!(rendered.contains("Finance a kapitálové trhy"));
        assert!(rendered.contains("Ze světa financí"));
        assert!(rendered.contains("Finance Article 1"));
        assert!(rendered.contains("Short text for finance 1"));
        assert!(rendered.contains("Wednesday, January 21, 2026"));

        Ok(())
    }
}
