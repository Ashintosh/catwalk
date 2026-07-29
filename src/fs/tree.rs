use std::path::Path;
use std::result::Result;

use crate::{error::CatwalkResult, fs::IgnoreRules, model::TreeNode};

pub fn build_tree(root: &Path, rules: &IgnoreRules) -> CatwalkResult<TreeNode> {
    let name = root
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("")
        .to_string();

    let mut node = TreeNode::new(name, root.to_path_buf(), true);

    build_children(&mut node, root, rules)?;

    Ok(node)
}

fn build_children(parent: &mut TreeNode, path: &Path, rules: &IgnoreRules) -> CatwalkResult<()> {
    let entries = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| !rules.should_ignore(&entry.path()))
        .collect::<Vec<_>>();

    let mut entries = entries;

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let entry_path = entry.path();
        let is_directory = entry.file_type()?.is_dir();

        let mut child = TreeNode::new(
            entry.file_name().to_string_lossy().to_string(),
            entry_path.clone(),
            is_directory,
        );

        if is_directory {
            build_children(&mut child, &entry_path, rules)?;
        }

        parent.add_child(child);
    }

    Ok(())
}
