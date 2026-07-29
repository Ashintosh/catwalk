use crate::{cli, error::CatwalkResult, export, fs};

use super::config::from_args;

pub fn run() -> CatwalkResult<()> {
    let args = cli::parse();
    cli::validate(&args)?;

    let rules = fs::IgnoreRules::new();
    let config = from_args(args);

    if config.print_tree {
        let tree = fs::build_tree(&config.root, &rules)?;
        print!("{}", export::format_tree(&tree));
    }

    let mut writer_box: Box<dyn export::OutputWriter> = match config.output.clone() {
        Some(path) => Box::new(export::FileWriter::new(path)?),
        None => Box::new(export::StdoutWriter::new()),
    };

    export::export_directory(&config.root, &rules, &mut *writer_box)?;

    Ok(())
}
