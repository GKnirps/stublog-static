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
use super::html::hosted_file;
use super::needs_any_update;
use crate::input::{Assets, HostedFile};
use std::fs::create_dir;
use std::io::Write;
use std::path::Path;

fn write_hosted_file_index_page(
    dir: &Path,
    files: &[HostedFile],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{}.html", current_page));
    if !needs_any_update(
        &filename,
        files
            .iter()
            .map(|f| f.modified_at)
            .chain(assets.modification_dates()),
    ) {
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        hosted_file::render_file_index_page(files, current_page, num_pages, assets).into_string()
    )
}

pub fn write_hosted_file_index_pages(
    dir: &Path,
    files: &[HostedFile],
    assets: &Assets,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    let chunk_size: usize = 45;
    let num_chunks = files.len() / chunk_size + if files.len() % chunk_size == 0 { 0 } else { 1 };

    for (index, chunk) in files.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_hosted_file_index_page(dir, chunk, index, num_chunks, assets)?;
    }

    Ok(())
}
