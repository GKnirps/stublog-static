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

use super::HostedFile;
use crate::input::ImageMetadata;
use image::io::Reader as ImageReader;
use quick_xml::events::Event;
use quick_xml::reader::Reader as XmlReader;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

// lists all files in the path and gets extra metadata on image files
pub fn list_all_files(path: &Path) -> std::io::Result<Vec<HostedFile>> {
    read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_size = metadata.len();
            let modified_at = metadata.modified()?;
            let mut filename = PathBuf::with_capacity(64);
            let entry_path = entry.path();
            filename.set_file_name(entry_path.file_name().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "No filename given for dir entry")
            })?);
            let file_extension = entry_path.extension().and_then(|ext| ext.to_str());
            // we take a shortcut here and rely on the file extensions, this makes things a little faster
            let image_metadata = if file_extension
                .map(|ext| ["png", "gif", "jpg", "jpeg", "webp"].contains(&ext))
                .unwrap_or(false)
            {
                read_image_metadata(&entry_path)
            } else if file_extension == Some("svg") {
                read_svg_size(&entry_path)
            } else {
                None
            };
            Ok(HostedFile {
                filename,
                file_size,
                modified_at,
                image_metadata,
            })
        })
        .collect()
}

// read image metadata from a file
// return None for any error (most importantly if the file is not a supported image format)
fn read_image_metadata(path: &Path) -> Option<ImageMetadata> {
    ImageReader::open(path)
        .ok()?
        .into_dimensions()
        .map(|(width, height)| ImageMetadata { width, height })
        .ok()
}

fn read_svg_size(path: &Path) -> Option<ImageMetadata> {
    let file = File::open(path).ok()?;
    let buf_reader = BufReader::new(file);
    parse_svg_size(buf_reader)
}

// FIXME: handle SVG without size without error
// FIXME: handle other than non-unit sizes without error
// FIXME: handle viewbox?
// FIXME: handle decimals in sizes
// try to read image sizes from an SVG. Return None on any parsing errors (most importantly if the
// file does not even vaguely look like an SVG
// This may break for unusual SVG files (or if the SVG file has no size information), because
// it makes some assumptions to make things easier
// it will also return None if not both width and height are present
// It is not required for SVG to have a specified width, so
fn parse_svg_size<T: BufRead>(source: T) -> Option<ImageMetadata> {
    let mut reader = XmlReader::from_reader(source);
    let mut buf: Vec<u8> = Vec::with_capacity(256);
    // let's just assume the SVG element has to be _somewhere_ at the document's start
    // if not… well, I have control over all SVG files here, so…
    for _ in 0..5 {
        if let Event::Start(tag) = reader.read_event_into(&mut buf).ok()? {
            if tag.name().into_inner() == b"svg" {
                let width_str = tag
                    .attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key.into_inner() == b"width")?
                    .value;
                let height_str = tag
                    .attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key.into_inner() == b"height")?
                    .value;
                let width: u32 = std::str::from_utf8(&width_str).ok()?.parse().ok()?;
                let height: u32 = std::str::from_utf8(&height_str).ok()?.parse().ok()?;
                return Some(ImageMetadata { width, height });
            }
        }
        buf.clear()
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_svg_size_should_read_unitless_size_of_valid_svg() {
        // given
        let svg = r##"<svg width="300" height="400" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let img = result.expect("expected image metadata");
        assert_eq!(img.width, 300);
        assert_eq!(img.height, 400);
    }

    #[test]
    fn read_svg_size_should_read_size_of_valid_svg_with_xml_header() {
        // given
        let svg = r##"<?xml version="1.0" encoding="UTF-8"?><svg width="300" height="400" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let img = result.expect("expected image metadata");
        assert_eq!(img.width, 300);
        assert_eq!(img.height, 400);
    }

    #[test]
    fn read_svg_size_should_work_case_insensitive() {
        // given
        let svg = r##"<svg width="300" height="400" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let img = result.expect("expected image metadata");
        assert_eq!(img.width, 300);
        assert_eq!(img.height, 400);
    }

    #[test]
    fn read_svg_size_should_return_none_if_width_is_missing() {
        // given
        let svg = r##"<svg height="400" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn read_svg_size_should_return_none_if_height_is_missing() {
        // given
        let svg = r##"<svg width="300" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn read_svg_size_should_fail_if_svg_tag_does_not_appear() {
        // given
        let svg = r##"<?xml version="1.0" encoding="UTF-8"?><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn read_svg_size_should_fail_for_bad_xml() {
        // given
        let svg = "Yo dawg I herd U liek XML";

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        assert_eq!(result, None);
    }
}
