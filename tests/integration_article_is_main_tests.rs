use axum::{
    body::Body,
    http::{Request, header},
};
use std::fs;
use tower::ServiceExt;

#[tokio::test]
async fn test_veda_article_is_main_rotation() {
    let (app, _db, cookie, original_index) = script_base::setup_test_environment().await;

    // Ensure index.html has known content in the sections
    let mut initial_index = original_index.clone();

    // Inject some identifiable content into MAIN, SECOND, THIRD
    let main_content = r#"
        <div class="main-article-text">
            <a href="old-main.html"><h1>Old Main Article</h1></a>
            <a href="old-main.html"><p>Old Main Short Text</p></a>
        </div>
        <a href="old-main.html"><img src="uploads/old-main.jpg" alt="Old Main Alt"></a>
    "#;
    let second_content = r#"
        <div class="first-article">
            <a href="old-second.html"><h2>Old Second Article</h2></a>
            <a href="old-second.html"><p>Old Second Short Text</p></a>
        </div>
    "#;
    let third_content = r#"
        <div class="second-article">
            <a href="old-third.html"><h2>Old Third Article</h2></a>
            <a href="old-third.html"><p>Old Third Short Text</p></a>
        </div>
    "#;

    if let (Some(s1), Some(e1)) = (
        initial_index.find("<!-- MAIN_ARTICLE -->"),
        initial_index.find("<!-- /MAIN_ARTICLE -->"),
    ) {
        initial_index.replace_range(s1 + "<!-- MAIN_ARTICLE -->".len()..e1, main_content);
    }
    if let (Some(s2), Some(e2)) = (
        initial_index.find("<!-- SECOND_ARTICLE -->"),
        initial_index.find("<!-- /SECOND_ARTICLE -->"),
    ) {
        initial_index.replace_range(s2 + "<!-- SECOND_ARTICLE -->".len()..e2, second_content);
    }
    if let (Some(s3), Some(e3)) = (
        initial_index.find("<!-- THIRD_ARTICLE -->"),
        initial_index.find("<!-- /THIRD_ARTICLE -->"),
    ) {
        initial_index.replace_range(s3 + "<!-- THIRD_ARTICLE -->".len()..e3, third_content);
    }
    fs::write("index.html", initial_index).unwrap();

    let boundary = "---------------------------123456789012345678901234567";
    let body = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        test-New Veda Main\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Author Veda\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        veda\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Main text of veda article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Short text of veda article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"is_main\"\r\n\r\n\
        on\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data\r\n\
        --{0}--\r\n",
        boundary
    );

    let _ = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    let updated_index = fs::read_to_string("index.html").unwrap();

    // Check MAIN_ARTICLE: should contain test-New Veda Main
    let main_start = updated_index.find("<!-- MAIN_ARTICLE -->").unwrap();
    let main_end = updated_index.find("<!-- /MAIN_ARTICLE -->").unwrap();
    let main_section = &updated_index[main_start..main_end];
    assert!(main_section.contains("test-New Veda Main"));
    assert!(main_section.contains("uploads/")); // Image should be there

    // Check SECOND_ARTICLE: should contain Old Main Article (stripped of image, class changed to first-article, h1 changed to h2)
    let second_start = updated_index.find("<!-- SECOND_ARTICLE -->").unwrap();
    let second_end = updated_index.find("<!-- /SECOND_ARTICLE -->").unwrap();
    let second_section = &updated_index[second_start..second_end];
    assert!(second_section.contains("Old Main Article"));
    assert!(
        second_section.contains("class=\"first-article\"")
            || second_section.contains("class='first-article'")
    );
    assert!(second_section.contains("<h2>Old Main Article</h2>"));
    assert!(!second_section.contains("<img")); // Image should be stripped

    // Check THIRD_ARTICLE: should contain Old Second Article (class changed to second-article)
    let third_start = updated_index.find("<!-- THIRD_ARTICLE -->").unwrap();
    let third_end = updated_index.find("<!-- /THIRD_ARTICLE -->").unwrap();
    let third_section = &updated_index[third_start..third_end];
    assert!(third_section.contains("Old Second Article"));
    assert!(
        third_section.contains("class=\"second-article\"")
            || third_section.contains("class='second-article'")
    );

    // Cleanup
    fs::write("index.html", original_index).unwrap();
    let _ = fs::remove_file("test-new-veda-main.html");
    let _ = fs::remove_file("snippets/test-new-veda-main.html.txt");
}
