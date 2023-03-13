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

use camino::{Utf8Path, Utf8PathBuf};
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::SystemTime;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct FileData {
    pub content: String,
    pub filename: Utf8PathBuf,
    pub modified_at: SystemTime,
}

pub fn read_files_sorted(dir_path: &Utf8Path) -> std::io::Result<Vec<FileData>> {
    let file_paths = read_sorted_dir(dir_path)?;
    file_paths.into_iter().map(read_file).collect()
}

pub fn read_sorted_dir(path: &Utf8Path) -> std::io::Result<Vec<Utf8PathBuf>> {
    let mut paths = Utf8Path::read_dir_utf8(path)?
        .map(|r| r.map(|entry| entry.into_path()))
        .filter(|r| {
            // only accept non-hidden files, but do not filter out errors (we want to know
            // if something bad happened)
            r.as_ref()
                .map(|p| !p.file_name().map(|f| f.starts_with('.')).unwrap_or(false))
                .unwrap_or(true)
        })
        .collect::<std::io::Result<Vec<Utf8PathBuf>>>()?;
    paths.sort_unstable();
    Ok(paths)
}

fn read_file(path: Utf8PathBuf) -> std::io::Result<FileData> {
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    let mut reader = BufReader::new(file);
    let mut content = String::with_capacity(metadata.len() as usize);
    reader.read_to_string(&mut content)?;

    Ok(FileData {
        content,
        filename: path,
        modified_at: metadata.modified()?,
    })
}
