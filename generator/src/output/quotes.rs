use super::file::open_for_write;
use super::html::quote::render_quote_page;
use super::needs_update;
use crate::input::Quote;
use std::fs::create_dir;
use std::io::Write;
use std::path::Path;

fn write_quote_page(dir: &Path, quote: &Quote) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&quote.filename);
    filename.set_extension("html");
    if !needs_update(&filename, quote.modified_at) {
        // target file is newer, no update needed
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(writer, "{}", render_quote_page(quote).into_string())
}

pub fn write_quote_pages(dir: &Path, quotes: &[Quote]) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for quote in quotes {
        // TODO: it would be more helpful if we knew which quote failed
        write_quote_page(dir, quote)?;
    }
    Ok(())
}
