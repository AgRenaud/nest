use nest::{settings, startup::Application};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    env_logger::init();

    let configuration = settings::get_settings(None).expect("Failed to read configuration.");

    let application = Application::build(configuration).await;

    application.run().await.unwrap();

    Ok(())
}
