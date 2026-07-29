use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "catwalk",
    version,
    about = "Format directory contents into a single output"
)]
pub struct Args {
    // Directory to export
    pub path: PathBuf,

    // Do not print directory tree
    #[arg(long)]
    pub no_tree: bool,

    // Additional directories to ignore
    #[arg(long, value_name = "DIR")]
    pub exclude: Vec<String>,

    // Write output to a file instead of stdout
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}
