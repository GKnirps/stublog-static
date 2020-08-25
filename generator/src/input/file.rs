use std::fs::{read_dir, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct FileData {
    pub content: String,
    pub filename: PathBuf,
    pub modified_at: SystemTime,
}

pub fn read_files_sorted(dir_path: &Path) -> std::io::Result<Vec<FileData>> {
    let file_paths = read_sorted_dir(dir_path)?;
    file_paths.into_iter().map(|path| read_file(path)).collect()
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

fn read_file(path: PathBuf) -> std::io::Result<FileData> {
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    let mut reader = BufReader::new(file);
    let mut content = String::with_capacity(2048);
    reader.read_to_string(&mut content)?;

    Ok(FileData {
        content,
        filename: path,
        modified_at: metadata.modified()?,
    })
}
