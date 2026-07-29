pub const SPECIAL_FILES: &[(&str, &str, &str)] = &[
    ("Dockerfile", "docker", "infrastructure"),
    ("docker-compose.yml", "yaml", "config"),
    ("Makefile", "make", "build"),
    ("Cargo.toml", "toml", "config"),
    ("package.json", "json", "config"),
    (".gitignore", "gitignore", "config"),
    (".env", "env", "config"),
];
