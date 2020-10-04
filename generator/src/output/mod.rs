use std::fs::metadata;
use std::path::Path;
use std::time::SystemTime;

pub mod blogposts;
pub mod categories;
mod cmark;
pub mod error_pages;
pub mod feed;
mod file;
mod html;
pub mod ngingx_cfg;
pub mod quotes;
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

/// Return true if the file reference by filename has an older modification date than any of
/// the given SystemTimes.
/// If any error occurs (e.g. the file does not exist), return true
fn needs_any_update<T>(filename: &Path, modification_times: T) -> bool
where
    T: Iterator<Item = SystemTime>,
{
    modification_times
        .max()
        .map(|t| needs_update(filename, t))
        .unwrap_or(true)
}
