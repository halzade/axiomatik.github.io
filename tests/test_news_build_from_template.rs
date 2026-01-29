#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form::form_index::{
        IndexCategoryArticleTemplate, IndexCategoryTemplate, NewsTemplate,
    };
    use axiomatik_web::system::system_data;

    #[test]
    fn test_news_build_from_template() {
        system_data::init_trivial_data();
        
        let empty_category = |name: &str, url: &str| IndexCategoryTemplate {
            category_name: name.to_string(),
            category_url: url.to_string(),
            articles: vec![],
        };

        let template = NewsTemplate {
            date: "Saturday, January 24, 2026".to_string(),
            weather: "-1°C | Prague".to_string(),
            name_day: "Milena".to_string(),
            articles_most_read: vec![],
            z_republiky: IndexCategoryTemplate {
                category_name: "Z naší republiky".to_string(),
                category_url: "republika.html".to_string(),
                articles: vec![IndexCategoryArticleTemplate {
                    url: "news-1.html".to_string(),
                    title: "Republika News 1".to_string(),
                    short_text: "Short text for republica".to_string(),
                    is_first: true,
                    image_path: "img.jpg".to_string(),
                    image_description: "image_description".to_string(),
                    category_name: "Republika".to_string(),
                    category_url: "republika.html".to_string(),
                }],
            },
            ze_zahranici: empty_category("Ze zahraničí", "zahranici.html"),
            technologie: empty_category("Ze světa technologií", "technologie.html"),
            veda: empty_category("Věda a výzkum", "veda.html"),
            finance: empty_category("Ze světa financí", "finance.html"),
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
