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

use super::HostedFile;
use crate::input::ImageMetadata;
use image::io::Reader as ImageReader;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

// lists all files in the path and gets extra metadata on image files
pub fn list_all_files(path: &Path) -> std::io::Result<Vec<HostedFile>> {
    read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_size = metadata.len();
            let modified_at = metadata.modified()?;
            let mut filename = PathBuf::with_capacity(64);
            let entry_path = entry.path();
            filename.set_file_name(entry_path.file_name().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "No filename given for dir entry")
            })?);
            // we take a shortcut here and rely on the file extensions, this makes things a little faster
            let image_metadata = if entry_path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ["png", "gif", "jpg", "jpeg", "webp"].contains(&ext))
                .unwrap_or(false)
            {
                read_image_metadata(&entry_path)
            } else {
                None
            };
            Ok(HostedFile {
                filename,
                file_size,
                modified_at,
                image_metadata,
            })
        })
        .collect()
}

// read image metadata from a file
// return None for any error (most importantly if the file is not a supported image format)
fn read_image_metadata(path: &Path) -> Option<ImageMetadata> {
    ImageReader::open(path)
        .ok()?
        .into_dimensions()
        .map(|(width, height)| ImageMetadata { width, height })
        .ok()
}
