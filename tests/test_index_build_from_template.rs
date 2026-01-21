#[cfg(test)]
mod tests {
    use askama::Template;
    use axiomatik_web::form_index::{
        IndexArticleTopData, IndexArticleTopMainData, IndexArticleTopMainTemplate,
        IndexArticleTopTemplate, IndexCategoryArticleData, IndexCategoryArticleTemplate,
        IndexCategoryData, IndexCategoryTemplate, IndexData, IndexTemplate,
    };

    #[test]
    fn test_index_build_from_template() {
        let index_data = IndexData {
            date: "Wednesday, January 21, 2026".to_string(),
            weather: "5°C | Prague".to_string(),
            name_day: "Bohdana".to_string(),
            main_article: IndexArticleTopMainData {
                url: "main-url".to_string(),
                title: "Main Title".to_string(),
                is_exclusive: false,
                short_text: "Main short text".to_string(),
                image_path: "img.jpg".to_string(),
            },
            second_article: IndexArticleTopData {
                url: "second-url".to_string(),
                title: "Second Title".to_string(),
                short_text: "Second short text".to_string(),
            },
            third_article: IndexArticleTopData {
                url: "third-url".to_string(),
                title: "Third Title".to_string(),
                short_text: "Third short text".to_string(),
            },
            z_republiky: IndexCategoryData {
                category_name: "Z naší republiky".to_string(),
                articles: vec![IndexCategoryArticleData {
                    url: "rep-1".to_string(),
                    title: "Rep 1".to_string(),
                    short_text: "Rep 1 text".to_string(),
                }],
            },
            ze_zahranici: IndexCategoryData {
                category_name: "Ze zahraničí".to_string(),
                articles: vec![IndexCategoryArticleData {
                    url: "for-1".to_string(),
                    title: "For 1".to_string(),
                    short_text: "For 1 text".to_string(),
                }],
            },
        };

        let template = IndexTemplate {
            date: index_data.date,
            weather: index_data.weather,
            name_day: index_data.name_day,
            main_article: IndexArticleTopMainTemplate {
                url: index_data.main_article.url,
                title: index_data.main_article.title,
                is_exclusive: false,
                short_text: index_data.main_article.short_text,
                image_path: index_data.main_article.image_path,
            },
            second_article: IndexArticleTopTemplate {
                url: index_data.second_article.url,
                title: index_data.second_article.title,
                short_text: index_data.second_article.short_text,
            },
            third_article: IndexArticleTopTemplate {
                url: index_data.third_article.url,
                title: index_data.third_article.title,
                short_text: index_data.third_article.short_text,
            },
            z_republiky: IndexCategoryTemplate {
                category_name: index_data.z_republiky.category_name,
                articles: index_data
                    .z_republiky
                    .articles
                    .into_iter()
                    .map(|a| IndexCategoryArticleTemplate {
                        url: a.url,
                        title: a.title,
                        short_text: a.short_text,
                    })
                    .collect(),
            },
            ze_zahranici: IndexCategoryTemplate {
                category_name: index_data.ze_zahranici.category_name,
                articles: index_data
                    .ze_zahranici
                    .articles
                    .into_iter()
                    .map(|a| IndexCategoryArticleTemplate {
                        url: a.url,
                        title: a.title,
                        short_text: a.short_text,
                    })
                    .collect(),
            },
        };

        let rendered = template.render().expect("Failed to render template");
        std::fs::write("test-index.html", &rendered).expect("Failed to write test-index.html");

        let saved_content =
            std::fs::read_to_string("test-index.html").expect("Failed to read test-index.html");

        // Basic content verification
        assert!(saved_content.contains("Main Title"));
        assert!(saved_content.contains("Second Title"));
        assert!(saved_content.contains("Third Title"));
        assert!(saved_content.contains("Rep 1"));
        assert!(saved_content.contains("For 1"));

        // HTML structure verification (un-escaped)
        assert!(saved_content.contains("<section class=\"main-article\">"));
        assert!(saved_content.contains("<div class=\"main-article-text\">"));
        // Cleanup
        let _ = std::fs::remove_file("test-index.html");
    }
}
