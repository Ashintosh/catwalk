use crate::model::TreeNode;

use super::tags;

pub fn format_tree(tree: &TreeNode) -> String {
    let mut output = String::new();

    output.push_str(&tags::open_tag(tags::PROJECT_TREE));
    output.push('\n');

    write_children(tree, "", &mut output);

    output.push_str(&tags::close_tag(tags::PROJECT_TREE));
    output.push('\n');

    output
}

fn write_children(node: &TreeNode, prefix: &str, output: &mut String) {
    let child_count = node.children.len();

    for (index, child) in node.children.iter().enumerate() {
        let is_last = index == child_count - 1;

        let connector = if is_last {
            "└──\x20"
        } else {
            "├──\x20"
        };

        let next_prefix = if is_last {
            format!("{prefix}\x20\x20\x20\x20")
        } else {
            format!("{prefix}│\x20\x20\x20")
        };

        output.push_str(prefix);
        output.push_str(connector);
        output.push_str(&child.name);
        output.push('\n');

        if child.is_directory {
            write_children(child, &next_prefix, output);
        }
    }
}
