/*
 * This file is part of stublog-static.
 *
 *  stublog-static is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  stublog-static is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with stublog-static. If not, see <https://www.gnu.org/licenses/>.
 */

use super::file::open_for_write;
use super::html::quote::render_quote_page;
use crate::HostedFile;
use crate::input::{Assets, Quote};
use crate::output::html::quote::render_quote_list_page;
use crate::output::{
    OutputError, hosted_files_modified_at_from_any_markdown,
    hosted_files_modified_at_from_markdown, needs_any_update,
};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::iter::once;

mod fortune;

fn write_quote_page(
    dir: &Utf8Path,
    quote: &Quote,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    let mut filename = dir.to_path_buf();
    filename.push(&quote.filename);
    filename.set_extension("html");

    let newest_hosted_file =
        hosted_files_modified_at_from_markdown(&quote.content_markdown, hosted_files)?;
    if !needs_any_update(
        &filename,
        assets
            .modification_dates()
            .iter()
            .copied()
            .chain(once(quote.modified_at))
            .chain(newest_hosted_file),
    ) {
        // target file is newer, no update needed
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        render_quote_page(quote, assets, hosted_files)?.into_string()
    )?;
    writer.flush().map_err(OutputError::from)
}

pub fn write_quote_pages(
    dir: &Utf8Path,
    quotes: &[Quote],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for quote in quotes {
        // TODO: it would be more helpful if we knew which quote failed
        write_quote_page(dir, quote, assets, hosted_files)?;
    }
    Ok(())
}

fn write_quote_list_page(
    dir: &Utf8Path,
    quotes: &[Quote],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
    index_updated: bool,
) -> Result<(), OutputError> {
    let filename = index_path(dir, current_page);

    let newest_hosted_file = hosted_files_modified_at_from_any_markdown(
        quotes.iter().map(|quote| &quote.content_markdown as &str),
        hosted_files,
    )?;

    if !needs_any_update(
        &filename,
        quotes
            .iter()
            .map(|q| q.modified_at)
            .chain(assets.modification_dates())
            .chain(newest_hosted_file),
    ) && !index_updated
    {
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        render_quote_list_page(quotes, current_page, num_pages, assets, hosted_files)?
            .into_string()
    )?;

    writer.flush().map_err(OutputError::from)
}

fn index_path(dir: &Utf8Path, page_index: usize) -> Utf8PathBuf {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{page_index}"));
    filename.set_extension("html");
    filename
}

pub fn write_quote_list_pages(
    dir: &Utf8Path,
    quotes: &[Quote],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    let chunk_size: usize = 50;
    let num_chunks = quotes.len() / chunk_size + usize::from(quotes.len() % chunk_size != 0);

    // we need this because when another page is added, all previous pages need to update because the
    // pager needs to include the new page
    let index_updated = if num_chunks > 0 {
        !index_path(dir, num_chunks - 1).exists()
    } else {
        true
    };

    for (index, chunk) in quotes.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_quote_list_page(
            dir,
            chunk,
            index,
            num_chunks,
            assets,
            hosted_files,
            index_updated,
        )?;
    }

    Ok(())
}

pub fn write_quote_fortune_file(dir: &Utf8Path, quotes: &[Quote]) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    let mut filename = dir.to_path_buf();
    filename.push("strangerthanusual");
    if !needs_any_update(&filename, quotes.iter().map(|q| q.modified_at)) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;

    fortune::write_fortune_quotes(&mut writer, quotes)?;

    writer.flush()
}
