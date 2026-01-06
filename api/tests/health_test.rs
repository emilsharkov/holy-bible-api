mod common;

use common::TestServer;

#[tokio::test]
async fn test_health_endpoint() {
    let server = TestServer::start()
        .await
        .expect("Failed to start test server");
    let client = server.client();

    let response = client
        .get(&format!("{}/health", server.base_url))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200, "Health endpoint should return 200");

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert_eq!(
        body.as_str(),
        Some("Healthy!"),
        "Health endpoint should return 'Healthy!'"
    );
}
