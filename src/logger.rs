use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub fn init_logger() -> anyhow::Result<()> {
    let format_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(format_layer)
        .with(filter_layer)
        .init();

    Ok(())
}
