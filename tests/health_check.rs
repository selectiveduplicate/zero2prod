use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};

use sqlx::{Connection, Executor, PgConnection, PgPool};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Launch the server in the background for testing.
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to get configuration");
    // Generate a random database name
    configuration.database_settings.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_test_database(&configuration.database_settings).await;
    let server =
        run(listener, connection_pool.clone()).expect("Failed to spawn app for running tests");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

async fn configure_test_database(config: &DatabaseSettings) -> PgPool {
    // Connect
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    // Create the random database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create random database for testing");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate after creating random database for testing");
    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // Spawn our app and take the address it's running on
    let testapp = spawn_app().await;

    // Create a client using reqwest
    let client = reqwest::Client::new();

    // Use the client to interact with the server and
    // get the response from the `/health_check` endpoint
    let response = client
        .get(&format!("{}/health_check", &testapp.address))
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
    let testapp = spawn_app().await;
    // Create a client using reqwest
    let client = reqwest::Client::new();
    // The body of the POST request
    let body = "name=fernando%20pessoa&email=fernando_pessoa%40gmail.com";

    // Use the client to interact with the server and
    // get the response from the `/subscriptions` endpoint
    // Assert the subscriber details
    let response = client
        .get(&format!("{}/subscriptions", &testapp.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        // Setting the body makes this a POST request
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert for desired response
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    // Fetch the subscriber details from the database
    let subscriber_details = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&testapp.db_pool)
        .await
        .expect("Failed to fetch subscription");

    // Assert the subscriber details
    assert_eq!(subscriber_details.email, "fernando_pessoa@gmail.com");
    assert_eq!(subscriber_details.name, "fernando pessoa");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    // Spawn our app and take the address it's running on
    let testapp = spawn_app().await;

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
            .get(&format!("{}/subscriptions", &testapp.address))
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
