use nest::settings;
use nest::startup::Application;
pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    let configuration = settings::Settings {
        application: settings::ApplicationSettings {
            host: String::from("127.0.0.1"),
            port: 0,
        },
        persistence: settings::PersistenceSettings {
            object_storage: settings::ObjectStorageSettings {
                path: String::from("./simple-index"),
            },
            database: settings::DatabaseSettings {
                address: String::from("127.0.0.1:8000"),
                user: String::from("root"),
                password: String::from("root"),
            },
        },
    };
    let application = Application::build(configuration).await;
    let address = format!("http://{}", application.address());
    let port = application.port();

    let _ = tokio::spawn(async move { application.run().await.expect("Failed to run the server") });

    TestApp { address, port }
}
