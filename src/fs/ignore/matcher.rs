use std::path::Path;

use super::defaults::DEFAULT_IGNORES;

#[derive(Debug, Clone)]
pub struct IgnoreRules {
    patterns: Vec<String>,
}

impl IgnoreRules {
    pub fn new() -> Self {
        Self {
            patterns: DEFAULT_IGNORES.iter().map(|v| v.to_string()).collect(),
        }
    }

    pub fn add(&mut self, pattern: impl Into<String>) {
        self.patterns.push(pattern.into());
    }

    pub fn should_ignore(&self, path: &Path) -> bool {
        path.components()
            .filter_map(|c| c.as_os_str().to_str())
            .any(|part| self.patterns.iter().any(|pattern| pattern == part))
    }
}
