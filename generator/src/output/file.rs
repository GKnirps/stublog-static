use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

pub fn open_for_write(filename: &Path) -> std::io::Result<BufWriter<File>> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .map(BufWriter::new)
}
