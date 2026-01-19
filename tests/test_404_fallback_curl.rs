#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_404_fallback_curl() {
        let output = std::process::Command::new("curl")
            .arg("-s")
            .arg("http://127.0.0.1:3000/non-existent-page.html")
            .output()
            .expect("Failed to execute curl");

        let body_str = String::from_utf8_lossy(&output.stdout);
        let expected_404_content = std::fs::read_to_string("404.html").unwrap();
        assert_eq!(body_str.trim(), expected_404_content.trim());
    }
}
