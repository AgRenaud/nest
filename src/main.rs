use nest::{
    settings,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = get_subscriber("nest".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    tracing::debug!("Read configuration");
    let configuration = settings::get_settings(None).expect("Failed to read configuration.");

    tracing::debug!("Build Application");
    let application = Application::build(configuration).await;

    tracing::debug!("Run Application");
    application.run().await;

    Ok(())
}
