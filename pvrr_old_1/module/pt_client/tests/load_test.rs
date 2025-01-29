use std::env;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

/// load test 会尝试加载所有配置
#[test]
fn load_test() {
    env::set_var("DATA_PATH", "../../test_data");
    env::set_var("RUST_LOG", "Trace");
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();
    pt_client::load();
}
