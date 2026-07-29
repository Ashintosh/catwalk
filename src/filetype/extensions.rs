pub const EXTENSION_TYPES: &[(&str, &str, &str)] = &[
    // Programming languages
    ("py", "python", "source"),
    ("rs", "rust", "source"),
    ("js", "javascript", "source"),
    ("jsx", "javascript", "source"),
    ("ts", "typescript", "source"),
    ("tsx", "typescript", "source"),
    ("java", "java", "source"),
    ("go", "go", "source"),
    ("c", "c", "source"),
    ("h", "c-header", "source"),
    ("cpp", "cpp", "source"),
    ("hpp", "cpp-header", "source"),
    ("cs", "csharp", "source"),
    ("swift", "swift", "source"),
    ("kt", "kotlin", "source"),
    // Shell
    ("sh", "bash", "script"),
    ("bash", "bash", "script"),
    ("zsh", "zsh", "script"),
    ("fish", "fish", "script"),
    // Web
    ("html", "html", "markup"),
    ("htm", "html", "markup"),
    ("css", "css", "style"),
    ("scss", "scss", "style"),
    // Configuration
    ("json", "json", "config"),
    ("yaml", "yaml", "config"),
    ("yml", "yaml", "config"),
    ("toml", "toml", "config"),
    ("ini", "ini", "config"),
    ("env", "env", "config"),
    // Documentation
    ("md", "markdown", "documentation"),
    ("txt", "text", "documentation"),
    // Data
    ("csv", "csv", "data"),
    ("sql", "sql", "data"),
    // Build/infrastructure
    ("dockerfile", "docker", "infrastructure"),
    ("tf", "terraform", "infrastructure"),
    // Images
    ("png", "image", "binary"),
    ("jpg", "image", "binary"),
    ("jpeg", "image", "binary"),
    ("gif", "image", "binary"),
    ("svg", "image", "binary"),
    // Documents
    ("pdf", "pdf", "document"),
];
