#[tokio::test]
async fn health_check_works() {
    // Spawn our app
    spawn_app();

    // Create a client using reqwest
    let client = reqwest::Client::new();

    // Use the client to interact with the server and
    // get the response from the `/health_check` endpoint
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

// Launch the server in the background for testing.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}