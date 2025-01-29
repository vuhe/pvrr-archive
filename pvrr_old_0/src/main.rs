use base_tool::error::AnyResult;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> AnyResult {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();

    database::load().await?;
    web_server::load().await?;

    Ok(())
}
