use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(name: impl Into<String>, path: PathBuf, is_directory: bool) -> Self {
        Self {
            name: name.into(),
            path,
            is_directory,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}
