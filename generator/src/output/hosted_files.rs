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
use crate::input::{Assets, HostedFileMetadata};
use crate::HostedFile;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs::create_dir;
use std::io::Write;

fn write_hosted_file_index_page(
    dir: &Utf8Path,
    files: &[(&HostedFileMetadata, &HostedFile)],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
    index_updated: bool,
) -> std::io::Result<()> {
    let filename = index_path(dir, current_page);
    if !needs_any_update(
        &filename,
        files
            .iter()
            .flat_map(|f| [f.0.modified_at, f.1.modified_at])
            .chain(assets.modification_dates()),
    ) && !index_updated
    {
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        hosted_file::render_file_index_page(files, current_page, num_pages, assets).into_string()
    )?;
    writer.flush()
}

fn index_path(dir: &Utf8Path, page_index: usize) -> Utf8PathBuf {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{page_index}"));
    filename.set_extension("html");
    filename
}

pub fn write_hosted_file_index_pages(
    dir: &Utf8Path,
    files: &[(&HostedFileMetadata, &HostedFile)],
    assets: &Assets,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    let chunk_size: usize = 45;
    let num_chunks = files.len() / chunk_size + usize::from(files.len() % chunk_size != 0);

    // we need this because when another page is added, all previous pages need to update because the
    // pager needs to include the new page
    let index_updated = if num_chunks > 0 {
        !index_path(dir, num_chunks - 1).exists()
    } else {
        true
    };

    for (index, chunk) in files.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_hosted_file_index_page(dir, chunk, index, num_chunks, assets, index_updated)?;
    }

    Ok(())
}
