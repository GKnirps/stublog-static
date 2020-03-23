use crate::parser::BlogpostMetadata;

pub fn blogpost_path(metadata: &BlogpostMetadata) -> String {
    format!("/blogpost/{}", metadata.filename.to_string_lossy())
}
