use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

mod parser;

fn main() -> Result<(), String> {
    let mut arg = args();
    arg.next();
    let infile = arg
        .next()
        .ok_or_else(|| "Test usage requires an input filename".to_owned())?;
    println!("Reading file {}", infile);
    let file_content =
        read_file(Path::new(&infile)).map_err(|e| format!("Error reading file: {}", e))?;

    let (header, content) =
        parser::parse_blogpost(&file_content).map_err(|e| format!("Error parsing file: {}", e))?;

    println!("Header: {:?}", header);
    println!("content size: {}", content.len());

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::with_capacity(2048);
    reader.read_to_string(&mut content)?;
    Ok(content)
}
