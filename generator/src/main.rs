#![feature(proc_macro_hygiene)]

use std::env::args;
use std::path::{Path, PathBuf};

mod blogposts;
mod file;
mod html;
mod parser;
mod paths;

fn main() -> Result<(), String> {
    let mut arg = args();
    let prog_name = arg.next().expect("Expected at least one argument");
    let indir = arg
        .next()
        .ok_or_else(|| format!("Usage: {} <input dir> <output dir>", prog_name))?;
    let odir = arg
        .next()
        .ok_or_else(|| format!("Usage: {} <input dir> <output dir>", prog_name))?;

    let blogpost_indir: PathBuf = [&indir, "blogposts"].iter().collect();
    let raw_blogposts = file::read_files_sorted(&Path::new(&blogpost_indir))
        .map_err(|e| format!("Failed to read all blogposts: {}", e))?;
    let blogposts = blogposts::parse_blogposts(&raw_blogposts)
        .map_err(|e| format!("Failed to parse all blogposts: {}", e))?;
    let blogpost_dir: PathBuf = [&odir, "blogpost"].iter().collect();
    blogposts::write_blogposts(&blogpost_dir, &blogposts)
        .map_err(|e| format!("Failed to write all blogposts: {}", e))?;

    // TODO: it does not really make sense to put this in blogposts. rename blogposts?
    blogposts::write_home(Path::new(&odir), &blogposts)
        .map_err(|e| format!("Failed to write home page: {}", e))
}
