use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Spawn our app and take the address it's running on
    let app_url = spawn_app();

    // Create a client using reqwest
    let client = reqwest::Client::new();

    // Use the client to interact with the server and
    // get the response from the `/health_check` endpoint
    let response = client
        .get(&format!("{}/health_check", &app_url))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

// Launch the server in the background for testing.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to spawn app for running tests");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}