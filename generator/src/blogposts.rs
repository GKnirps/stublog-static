use std::fs::{create_dir, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use super::html;
use super::parser;

pub fn parse_blogposts<'a>(
    inputs: &'a [String],
) -> Result<Vec<(parser::BlogpostMetadata, &'a str)>, parser::ParseError> {
    // TODO: it would be more helpful if we knew which blogpost failed to parse
    inputs
        .iter()
        .map(|input| parser::parse_blogpost(&input))
        .collect()
}

pub fn write_blogposts(
    dir: &Path,
    posts: &[(parser::BlogpostMetadata, &str)],
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for (post_metadata, post_content) in posts {
        // TODO: it would be more helpful if we knew which blogpost failed
        write_blogpost(dir, post_metadata, post_content)?;
    }
    Ok(())
}

fn write_blogpost(
    dir: &Path,
    metadata: &parser::BlogpostMetadata,
    content: &str,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&metadata.filename);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&filename)?;
    let mut writer = BufWriter::new(file);
    write!(
        writer,
        "{}",
        html::blogpost::render_blogpost(metadata, content).into_string()
    )
}
