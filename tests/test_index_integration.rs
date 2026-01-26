use axiomatik_web::test_framework::article_builder::ArticleBuilder;
use axiomatik_web::test_framework::script_base;
use axiomatik_web::test_framework::script_base::content_type_with_boundary;
use axiomatik_web::test_framework::script_base_data::PNG;
use axum::http::{header, StatusCode};
use axum_core::extract::Request;
use reqwest::Body;
use std::fs;

#[tokio::test]
async fn test_index_main_article() {
    script_base::setup_before_tests_once().await;
    let cookie = script_base::setup_user_and_login("user_main").await;

    let image_data = script_base::get_test_image_data();
    let body = ArticleBuilder::new()
        .title("Regular Main Article")
        .author("Author")
        .category("republika")
        .text("Text")
        .short_text("Short")
        .main()
        .image("img.jpg", &image_data, PNG)
        .image_description("desc")
        .build()
        .unwrap();

    let resp = script_base::one_shot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, content_type_with_boundary())
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let index_html = fs::read_to_string("index.html").unwrap();
    assert!(index_html.contains("Regular Main Article"));
    assert!(!index_html.contains("<span class=\"red\">EXKLUZIVNĚ:</span>"));

    // Cleanup
    let _ = fs::remove_file("web/regular-main-article.html");
    let _ = fs::remove_file("web/uploads/regular-main-article_image_820.jpg");
    let _ = fs::remove_file("web/uploads/regular-main-article_image_50.jpg");
    let _ = fs::remove_file("web/uploads/regular-main-article_image_288.jpg");
    let _ = fs::remove_file("web/uploads/regular-main-article_image_440.jpg");
}

#[tokio::test]
async fn test_index_main_article_exclusive() {
    script_base::setup_before_tests_once().await;
    let cookie = script_base::setup_user_and_login("user_exclusive").await;

    let image_data = script_base::get_test_image_data();
    let body = ArticleBuilder::new()
        .title("Exclusive Main Article")
        .author("Author")
        .category("republika")
        .text("Text")
        .short_text("Short")
        .main()
        .exclusive()
        .image("img_ex.jpg", &image_data, PNG)
        .image_description("desc")
        .build()
        .unwrap();

    let resp = script_base::one_shot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, content_type_with_boundary())
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let index_html = fs::read_to_string("index.html").unwrap();
    assert!(index_html.contains("Exclusive Main Article"));
    assert!(index_html.contains("<span class=\"red\">EXKLUZIVNĚ:</span>"));

    // Cleanup
    let _ = fs::remove_file("web/exclusive-main-article.html");
    let _ = fs::remove_file("web/uploads/exclusive-main-article_image_820.jpg");
    let _ = fs::remove_file("web/uploads/exclusive-main-article_image_50.jpg");
    let _ = fs::remove_file("web/uploads/exclusive-main-article_image_288.jpg");
    let _ = fs::remove_file("web/uploads/exclusive-main-article_image_440.jpg");
}

#[tokio::test]
async fn test_index_article_republika() {
    script_base::setup_before_tests_once().await;
    let cookie = script_base::setup_user_and_login("user_rep").await;

    let image_data = script_base::get_test_image_data();
    let body = ArticleBuilder::new()
        .title("New Republika Article")
        .author("Author")
        .category("republika")
        .text("Text")
        .short_text("Short")
        .image("img_rep.jpg", &image_data, PNG)
        .image_description("desc")
        .build()
        .unwrap();

    let resp = script_base::one_shot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, content_type_with_boundary())
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let index_html = fs::read_to_string("index.html").unwrap();
    // Republika section header
    assert!(index_html.contains("> Z naší republiky"));
    assert!(index_html.contains("New Republika Article"));

    // Cleanup
    let _ = fs::remove_file("web/new-republika-article.html");
    let _ = fs::remove_file("web/uploads/new-republika-article_image_820.jpg");
    let _ = fs::remove_file("web/uploads/new-republika-article_image_50.jpg");
    let _ = fs::remove_file("web/uploads/new-republika-article_image_288.jpg");
    let _ = fs::remove_file("web/uploads/new-republika-article_image_440.jpg");
}

#[tokio::test]
async fn test_index_article_zahranici() {
    script_base::setup_before_tests_once().await;
    let cookie = script_base::setup_user_and_login("user_zah").await;

    let image_data = script_base::get_test_image_data();
    let body = ArticleBuilder::new()
        .title("New Zahranici Article")
        .author("Author")
        .category("zahranici")
        .text("Text")
        .short_text("Short")
        .image("img_zah.jpg", &image_data, PNG)
        .image_description("desc")
        .build()
        .unwrap();

    let resp = script_base::one_shot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, content_type_with_boundary())
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let index_html = fs::read_to_string("index.html").unwrap();
    // Zahranici section header
    assert!(index_html.contains("> Ze zahraničí"));
    assert!(index_html.contains("New Zahranici Article"));

    // Cleanup
    let _ = fs::remove_file("web/new-zahranici-article.html");
    let _ = fs::remove_file("web/uploads/new-zahranici-article_image_820.jpg");
    let _ = fs::remove_file("web/uploads/new-zahranici-article_image_50.jpg");
    let _ = fs::remove_file("web/uploads/new-zahranici-article_image_288.jpg");
    let _ = fs::remove_file("web/uploads/new-zahranici-article_image_440.jpg");
}

#[tokio::test]
async fn test_index_article_veda() {
    script_base::setup_before_tests_once().await;
    let cookie = script_base::setup_user_and_login("user_veda").await;

    let image_data = script_base::get_test_image_data();
    let body = ArticleBuilder::new()
        .title("New Veda Article")
        .author("Author")
        .category("veda")
        .text("Text")
        .short_text("Short")
        .image("img_veda.jpg", &image_data, PNG)
        .image_description("desc")
        .build()
        .unwrap();

    let resp = script_base::one_shot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, content_type_with_boundary())
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let index_html = fs::read_to_string("index.html").unwrap();

    // Find section starts
    let rep_start = index_html.find("> Z naší republiky").unwrap();
    let zah_start = index_html.find("> Ze zahraničí").unwrap();

    // Check it's NOT in these sections
    // In our template, Republika comes before Zahranici
    let rep_section = &index_html[rep_start..zah_start];
    let zah_section = &index_html[zah_start..];

    assert!(!rep_section.contains("New Veda Article"));
    assert!(!zah_section.contains("New Veda Article"));

    // Cleanup
    let _ = fs::remove_file("web/new-veda-article.html");
    let _ = fs::remove_file("web/uploads/new-veda-article_image_820.jpg");
    let _ = fs::remove_file("web/uploads/new-veda-article_image_50.jpg");
    let _ = fs::remove_file("web/uploads/new-veda-article_image_288.jpg");
    let _ = fs::remove_file("web/uploads/new-veda-article_image_440.jpg");
}
