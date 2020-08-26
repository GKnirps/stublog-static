use std::fs::metadata;
use std::path::Path;
use std::time::SystemTime;

pub mod blogposts;
pub mod categories;
mod cmark;
mod file;
mod html;
pub mod tags;

/// Return true if the file referenced by filename has an older modification date than modified_at.
/// If any error occurs (e.g. the file does not exist), return true.
/// otherwise return false
fn needs_update(filename: &Path, modified_at: SystemTime) -> bool {
    metadata(filename)
        .and_then(|m| m.modified())
        .map(|t| t < modified_at)
        .unwrap_or(true)
}
