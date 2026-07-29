use crate::model::ExportedFile;

use super::tags;

pub fn format_file(file: &ExportedFile) -> String {
    let path = file.relative_path.to_string_lossy().replace('\\', "/");

    let file_tag_begin =
        tags::open_tag_with_attributes(tags::FILE, &[("path", &path), ("type", &file.file_type)]);

    format!(
        r#"{file_tag_begin}
{content_begin}
{content}
{content_end}
{file_tag_end}
"#,
        content_begin = tags::open_tag(tags::CONTENT),
        content = file.content,
        content_end = tags::close_tag(tags::CONTENT),
        file_tag_end = tags::close_tag(tags::FILE)
    )
}
