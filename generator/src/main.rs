#![feature(proc_macro_hygiene)]

use std::env::args;
use std::fs::{create_dir, read_dir, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

mod html;
mod parser;

fn main() -> Result<(), String> {
    let mut arg = args();
    let prog_name = arg.next().expect("Expected at least one argument");
    let indir = arg
        .next()
        .ok_or_else(|| format!("Usage: {} <input dir> <output dir>", prog_name))?;
    let odir = arg
        .next()
        .ok_or_else(|| format!("Usage: {} <input dir> <output dir>", prog_name))?;

    let blogpost_indir: PathBuf = [&indir, "blogpost"].iter().collect();
    let raw_blogposts = read_files_sorted(&Path::new(&blogpost_indir))
        .map_err(|e| format!("Failed to read all blogposts: {}", e))?;
    let blogposts = parse_blogposts(&raw_blogposts)
        .map_err(|e| format!("Failed to parse all blogposts: {}", e))?;
    let blogpost_dir: PathBuf = [&odir, "blogpost"].iter().collect();
    write_blogposts(&blogpost_dir, &blogposts)
        .map_err(|e| format!("Failed to write all blogposts: {}", e))
}

fn read_files_sorted(dir_path: &Path) -> std::io::Result<Vec<String>> {
    let file_paths = read_sorted_dir(dir_path)?;
    file_paths.iter().map(|path| read_file(path)).collect()
}

fn read_sorted_dir(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths = read_dir(path)?
        .map(|r| r.map(|entry| entry.path()))
        .filter(|r| {
            // only accept non-hidden files, but do not filter out errors (we want to know
            // if something bad happened)
            r.as_ref()
                .map(|p| {
                    !p.file_name()
                        .and_then(|f| f.to_str())
                        .map(|f| f.starts_with('.'))
                        .unwrap_or(false)
                })
                .unwrap_or(true)
        })
        .collect::<std::io::Result<Vec<PathBuf>>>()?;
    paths.sort_unstable();
    Ok(paths)
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::with_capacity(2048);
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_blogposts<'a>(
    inputs: &'a [String],
) -> Result<Vec<(parser::BlogpostMetadata, &'a str)>, parser::ParseError> {
    // TODO: it would be more helpful if we knew which blogpost failed to parse
    inputs
        .iter()
        .map(|input| parser::parse_blogpost(&input))
        .collect()
}

fn write_blogposts(dir: &Path, posts: &[(parser::BlogpostMetadata, &str)]) -> std::io::Result<()> {
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
