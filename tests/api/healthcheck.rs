use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    println!("{}", &app.address);

    let response = client
        .get(format!("{}/healthcheck", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    println!("{}", &response.status());
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
