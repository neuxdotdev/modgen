mod cli;
mod config;
mod fs_utils;
mod generator;
mod logging;
mod scanner;
use anyhow::Result;
use clap::Parser;
use cli::Cli;
use tracing::info;
fn main() -> Result<()> {
    let cli = Cli::parse();
    logging::setup_logging(cli.verbose)?;
    info!("Starting modgen");
    let config = config::load_config(cli.config.as_ref())?;
    let generator = generator::Generator::new(config, cli.dry_run, cli.no_reexport, cli.check);
    generator.run(&cli.path)?;
    info!("Done");
    Ok(())
}
