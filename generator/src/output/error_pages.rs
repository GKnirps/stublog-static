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
