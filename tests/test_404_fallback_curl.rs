#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;
    use tracing::{error, info};
    use axiomatik_web::{configuration, server};

    #[tokio::test]
    async fn test_404_fallback_curl() {

        // init test
        let addr = "127.0.0.1:3025";
        let router = server::start_router().await;
        info!("listening on {}", &addr);
        let listener = TcpListener::bind(&addr)
            .await
            .expect(&format!("Failed to bind to {}", &addr));
        if let Err(err) = axum::serve(listener, router).await {
            error!("axum server exited: {:?}", err);
        };

        let output = std::process::Command::new("curl")
            .arg("-s")
            .arg(format!("http://{}/non-existent-page.html", addr))
            .output()
            .expect("Failed to execute curl");

        let body_str = String::from_utf8_lossy(&output.stdout);
        let expected_404_content = std::fs::read_to_string("404.html").unwrap();
        assert_eq!(body_str.trim(), expected_404_content.trim());
    }
}
