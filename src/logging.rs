use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt};
pub fn setup_logging(verbosity: u8) -> Result<()> {
    let level = match verbosity {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .map_err(|e| anyhow::anyhow!("invalid log filter: {}", e))?;
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| anyhow::anyhow!("logging already initialized"))?;
    Ok(())
}
