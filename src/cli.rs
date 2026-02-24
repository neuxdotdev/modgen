use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value = "src")]
    pub path: PathBuf,
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub no_reexport: bool,
    #[arg(long)]
    pub check: bool,
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
