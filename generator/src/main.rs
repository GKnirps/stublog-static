#![feature(proc_macro_hygiene)]

use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

mod html;
mod parser;

fn main() -> Result<(), String> {
    let mut arg = args();
    arg.next();
    let infile = arg
        .next()
        .ok_or_else(|| "Test usage requires an input filename".to_owned())?;
    let odir = arg.next().ok_or_else(|| {
        "Test usage requires an output directory after the input filename".to_owned()
    })?;
    println!("Reading file {}", infile);
    let file_content =
        read_file(Path::new(&infile)).map_err(|e| format!("Error reading file: {}", e))?;

    let (header, content) =
        parser::parse_blogpost(&file_content).map_err(|e| format!("Error parsing file: {}", e))?;

    write_blogpost(Path::new(&odir), &header, content)
        .map_err(|e| format!("Error writing file: {}", e))
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::with_capacity(2048);
    reader.read_to_string(&mut content)?;
    Ok(content)
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
