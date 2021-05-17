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

use std::fs::metadata;
use std::path::Path;
use std::time::SystemTime;

pub mod blogposts;
pub mod categories;
mod cmark;
pub mod error_pages;
pub mod feed;
mod file;
pub mod hosted_files;
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
    T: IntoIterator<Item = SystemTime>,
{
    modification_times
        .into_iter()
        .max()
        .map(|t| needs_update(filename, t))
        .unwrap_or(true)
}
