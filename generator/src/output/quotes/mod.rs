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
use super::needs_any_update;
use crate::input::{Assets, Quote};
use crate::output::html::quote::render_quote_list_page;
use crate::output::OutputError;
use crate::HostedFile;
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::path::Path;

mod fortune;

fn write_quote_page(
    dir: &Path,
    quote: &Quote,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    let mut filename = dir.to_path_buf();
    filename.push(&quote.filename);
    filename.set_extension("html");
    // FIXME: check if hosted files changed
    if !needs_any_update(
        &filename,
        assets
            .modification_dates()
            .iter()
            .copied()
            .chain(Some(quote.modified_at)),
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
    writer
        .into_inner()
        .map_err(OutputError::from)?
        .sync_all()
        .map_err(OutputError::from)
}

pub fn write_quote_pages(
    dir: &Path,
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
    dir: &Path,
    quotes: &[Quote],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{}.html", current_page));
    // FIXME: check if hosted files changed
    if !needs_any_update(
        &filename,
        quotes
            .iter()
            .map(|q| q.modified_at)
            .chain(assets.modification_dates()),
    ) {
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        render_quote_list_page(quotes, current_page, num_pages, assets, hosted_files)?
            .into_string()
    )?;

    writer
        .into_inner()
        .map_err(OutputError::from)?
        .sync_all()
        .map_err(OutputError::from)
}

pub fn write_quote_list_pages(
    dir: &Path,
    quotes: &[Quote],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    let chunk_size: usize = 50;
    let num_chunks = quotes.len() / chunk_size + usize::from(quotes.len() % chunk_size == 0);

    for (index, chunk) in quotes.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_quote_list_page(dir, chunk, index, num_chunks, assets, hosted_files)?;
    }

    Ok(())
}

pub fn write_quote_fortune_file(dir: &Path, quotes: &[Quote]) -> std::io::Result<()> {
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

    writer.into_inner()?.sync_all()
}
