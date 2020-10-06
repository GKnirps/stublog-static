use super::file::open_for_write;
use crate::input::Blogpost;
use crate::output::needs_any_update;
use quick_xml::Writer;
use std::path::Path;

mod atom;

pub fn feed_needs_update(filename: &Path, blogposts: &[Blogpost]) -> bool {
    let times = blogposts.iter().map(|p| p.modified_at);
    needs_any_update(filename, times)
}

pub fn write_atom_feed(dir: &Path, blogposts: &[Blogpost]) -> Result<(), String> {
    let mut filename = dir.to_path_buf();
    filename.push("feed.atom");
    if !feed_needs_update(&filename, blogposts) {
        return Ok(());
    }

    let file =
        open_for_write(&filename).map_err(|e| format!("Unable to open atom feed file: {}", e))?;
    let mut writer = Writer::new(file);

    atom::write_feed(&mut writer, blogposts)
        .map_err(|e| format!("Unable to write atom feed: {}", e))
}
