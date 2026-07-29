pub const DEFAULT_IGNORES: &[&str] = &[
    // Version control metadata
    ".git",
    ".svn",
    ".hg",
    // Dependency caches / installed packages
    "node_modules",
    // Common build outputs
    "target",
    "dist",
    // Python cache artifacts
    "__pycache__",
    // Common virtual environments
    ".venv",
    "venv",
    // IDE metadata
    ".idea",
    ".vscode",
];
