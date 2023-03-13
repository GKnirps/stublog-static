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

use camino::Utf8Path;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;

pub fn open_for_write(filename: &Utf8Path) -> std::io::Result<BufWriter<File>> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .map(BufWriter::new)
}
