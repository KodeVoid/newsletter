use std::net::TcpListener;
use newsletter::run; // or whatever your crate name is

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(Some(listener)).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

// Async version with better error handling
pub async fn spawn_app_async() -> Result<String, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = run(Some(listener))?;
    tokio::spawn(server);

    // Give server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    Ok(address)
}
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app_async().await.expect("Failed to spawn app");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/health", address))
        .header("X-Route-Type", "health")
        .send().await
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
