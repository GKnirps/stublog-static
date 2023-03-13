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

use super::super::{Asset, Assets};
use crate::input::parser::ParseError;
use camino::{Utf8Path, Utf8PathBuf};
use sha2::{digest, Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

fn read_asset(path: Utf8PathBuf) -> Result<Asset, ParseError> {
    let file = File::open(&path).map_err(|e| {
        ParseError::new(format!(
            "Unable to open asset file {}: {}",
            path.as_str(),
            e
        ))
    })?;
    let metadata = file.metadata().map_err(|e| {
        ParseError::new(format!(
            "Unable get metadata from asset file {}: {}",
            path.as_str(),
            e
        ))
    })?;
    let mut reader = BufReader::new(file);

    let mut content: Vec<u8> = Vec::with_capacity(metadata.len() as usize);
    reader.read_to_end(&mut content).map_err(|e| {
        ParseError::new(format!(
            "Unable to read asset file {}: {}",
            path.as_str(),
            e
        ))
    })?;

    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hasher.finalize();

    let web_path = format_web_path(&path, &hash).ok_or_else(|| {
        ParseError::new(format!(
            "Unable to create web path for asset {}",
            path.as_str()
        ))
    })?;

    let modified_at = metadata.modified().map_err(|e| {
        ParseError::new(format!(
            "Unable to get modification time for asset file {}: {}",
            path.as_str(),
            e
        ))
    })?;

    Ok(Asset {
        filename: path,
        modified_at,
        web_path,
    })
}

fn format_web_path(file_path: &Utf8Path, hash: &digest::Output<Sha256>) -> Option<String> {
    Some(format!(
        "/assets/{}?cache={:x}",
        file_path.file_name()?,
        hash
    ))
}

/// read asset files (from the asset output directory to hash them (for caching)
pub fn read_assets(asset_dir: &Utf8Path) -> Result<Assets, ParseError> {
    let mut favicon_path = asset_dir.to_path_buf();
    favicon_path.push("favicon.png");
    let favicon = read_asset(favicon_path)?;

    let mut stylesheet_path = asset_dir.to_path_buf();
    stylesheet_path.push("style.css");
    let stylesheet = read_asset(stylesheet_path)?;

    Ok(Assets {
        favicon,
        stylesheet,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_web_path_formats_correctly() {
        // given
        let file_path = Utf8Path::new("../somewhere/bar/foo.css");

        let mut hasher = Sha256::new();
        hasher.update(b"Look behind you, a three-headed monkey!");
        let hash = hasher.finalize();

        // when
        let result = format_web_path(file_path, &hash).expect("Expected a path");

        // then
        assert_eq!(&result, "/assets/foo.css?cache=8b15b53224ad352e142008a0821464f2988bc1462d0350f11ce0ca414069bb81")
    }
}
