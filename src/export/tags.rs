pub const EXPORT: &str = "CATWALK_EXPORT";
pub const PROJECT_TREE: &str = "PROJECT_TREE";
pub const FILE: &str = "FILE";
pub const CONTENT: &str = "CONTENT";

pub fn open_tag(name: &str) -> String {
    format!("<{name}>")
}

pub fn close_tag(name: &str) -> String {
    format!("</{name}>")
}

pub fn open_tag_with_attributes(name: &str, attributes: &[(&str, &str)]) -> String {
    let attrs = attributes
        .iter()
        .map(|(key, value)| format!(r#" {key}="{value}""#))
        .collect::<String>();

    format!("<{name}{attrs}>")
}
