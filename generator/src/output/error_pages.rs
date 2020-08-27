use super::file::open_for_write;
use super::html;
use std::path::Path;

use std::io::Write;

pub fn write_404(base_path: &Path) -> std::io::Result<()> {
    let mut filename = base_path.to_path_buf();
    filename.push("404.html");

    // Only write this file if it does not exist already.
    // If changes are made to the generation, the existing file needs to be deleted
    if filename.exists() {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;
    write!(writer, "{}", html::error_pages::render_404().into_string())
}
