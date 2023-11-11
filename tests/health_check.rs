use std::net::TcpListener;
use zero2prod::{startup::run, configuration::get_configuration};

use sqlx::{Connection, PgConnection};

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Spawn our app and take the address it's running on
    let app_url = spawn_app();
    // Create a client using reqwest
    let client = reqwest::Client::new();
    // The body of the POST request
    let body = "name=fernando%20pessoa&email=fernando_pessoa%40gmail.com";

    // Try to connect to the Postgres database
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database_settings.connection_string();
    let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to postgres");

    // Use the client to interact with the server and
    // get the response from the `/subscriptions` endpoint
    let response = client
        .get(&format!("{}/subscriptions", &app_url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        // Setting the body makes this a POST request
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    let subscriber_details = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&mut connection)
            .await
            .expect("Failed to fetch subscription");

    assert_eq!(subscriber_details.email, "fernando_pessoa@gmail.com");
    assert_eq!(subscriber_details.name, "fernando pessoa");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    // Spawn our app and take the address it's running on
    let app_url = spawn_app();

    // Create a client using reqwest
    let client = reqwest::Client::new();

    // The body of the POST request containing invalid form data testcases
    let test_cases = vec![
        ("name=fernando%20pessoa", "email is missing"),
        ("email=fernando_pessoa%40gmail.com", "name is missing"),
        ("", "missing both name and email"),
    ]; 

    for (invalid_body, err_msg) in test_cases {
        let response = client
            .get(&format!("{}/subscriptions", &app_url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Custom error message on test failure
            "The API did not fail with a 400 Bad Request when the payload was {}",
            err_msg
        );
    }
}

// Launch the server in the background for testing.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to spawn app for running tests");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}