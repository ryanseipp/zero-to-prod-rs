use zero_to_prod_rs::configuration::get_configuration;
use zero_to_prod_rs::startup::Application;
use zero_to_prod_rs::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("zero-to-prod-rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
