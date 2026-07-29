use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    // Root directory being exported
    pub root: PathBuf,

    // Directories to ignore in addition to the defaults
    pub exclude: Vec<String>,

    // Follow symbolic links while walking
    pub follow_symlinks: bool,

    // Print the directory tree before exporting files
    pub print_tree: bool,

    // Optional output file
    pub output: Option<PathBuf>,
}

impl Config {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            exclude: Vec::new(),
            follow_symlinks: false,
            print_tree: true,
            output: None,
        }
    }
}
