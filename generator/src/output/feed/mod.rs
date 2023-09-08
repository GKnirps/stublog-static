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
use crate::input::Blogpost;
use crate::output::needs_any_update;
use crate::HostedFile;
use camino::Utf8Path;
use quick_xml::Writer;
use std::collections::HashMap;
use std::io::Write;

mod atom;

pub fn feed_needs_update(filename: &Utf8Path, blogposts: &[Blogpost]) -> bool {
    let times = blogposts.iter().map(|p| p.modified_at);
    needs_any_update(filename, times)
}

pub fn write_atom_feed(
    dir: &Utf8Path,
    blogposts: &[Blogpost],
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), String> {
    let mut filename = dir.to_path_buf();
    filename.push("feed.atom");
    if !feed_needs_update(&filename, blogposts) {
        return Ok(());
    }

    let file =
        open_for_write(&filename).map_err(|e| format!("Unable to open atom feed file: {e}"))?;
    let mut writer = Writer::new(file);

    atom::write_feed(&mut writer, blogposts, hosted_files)
        .map_err(|e| format!("Unable to write atom feed: {e}"))?;

    writer
        .into_inner()
        .flush()
        .map_err(|e| format!("Unable to write atom feed: {e}"))
}
