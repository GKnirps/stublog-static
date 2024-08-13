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
use camino::{Utf8Path, Utf8PathBuf};
use image::io::Reader as ImageReader;
use quick_xml::events::Event;
use quick_xml::reader::Reader as XmlReader;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum ListFilesError {
    Io(std::io::Error),
    MissingFileName(Utf8PathBuf),
    RasterImage(Utf8PathBuf, image::error::ImageError),
    Svg(Utf8PathBuf, String, Option<quick_xml::Error>),
}

impl Error for ListFilesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ListFilesError::Io(e) => Some(e),
            ListFilesError::MissingFileName(_) => None,
            ListFilesError::RasterImage(_, e) => Some(e),
            ListFilesError::Svg(_, _, e) => Some(e.as_ref()?),
        }
    }
}

impl fmt::Display for ListFilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListFilesError::Io(e) => e.fmt(f),
            ListFilesError::MissingFileName(path) => {
                write!(f, "No file name for directory entry '{path}'")
            }
            ListFilesError::RasterImage(path, e) => {
                write!(f, "unable to get image metadata from '{path}': {e}")
            }
            ListFilesError::Svg(path, msg, xmle) => {
                if let Some(e) = xmle {
                    write!(f, "unable to get image metadata from '{path}': {e}")
                } else {
                    write!(f, "unable to get image metadata from '{path}': {msg}")
                }
            }
        }
    }
}

impl From<std::io::Error> for ListFilesError {
    fn from(e: std::io::Error) -> Self {
        ListFilesError::Io(e)
    }
}

// lists all files in the path and gets extra metadata on image files
pub fn list_all_files(path: &Utf8Path) -> Result<Vec<HostedFile>, ListFilesError> {
    Utf8Path::read_dir_utf8(path)?
        .map(|entry| {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_size = metadata.len();
            let modified_at = metadata.modified()?;
            let mut filename = Utf8PathBuf::with_capacity(64);
            let entry_path = entry.path();
            filename.set_file_name(
                entry_path
                    .file_name()
                    .ok_or_else(|| ListFilesError::MissingFileName(entry_path.to_owned()))?,
            );
            let file_extension = entry_path.extension();
            // we take a shortcut here and rely on the file extensions, this makes things a little faster
            let image_metadata = if file_extension
                .map(|ext| ["png", "gif", "jpg", "jpeg", "webp"].contains(&ext))
                .unwrap_or(false)
            {
                Some(read_image_metadata(entry_path)?)
            } else if file_extension == Some("svg") {
                read_svg_size(entry_path)?
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
fn read_image_metadata(path: &Utf8Path) -> Result<ImageMetadata, ListFilesError> {
    ImageReader::open(path)
        .map_err(|e| ListFilesError::RasterImage(path.to_owned(), e.into()))?
        .into_dimensions()
        .map(|(width, height)| ImageMetadata { width, height })
        .map_err(|e| ListFilesError::RasterImage(path.to_owned(), e))
}

fn read_svg_size(path: &Utf8Path) -> Result<Option<ImageMetadata>, ListFilesError> {
    let file = File::open(path)
        .map_err(|e| ListFilesError::Svg(path.to_owned(), String::new(), Some(e.into())))?;
    let buf_reader = BufReader::new(file);
    parse_svg_size(buf_reader).map_err(|e| match e {
        SvgError::Xml(e) => ListFilesError::Svg(path.to_owned(), String::new(), Some(e)),
        SvgError::Svg(s) => ListFilesError::Svg(path.to_owned(), s, None),
    })
}

#[derive(Debug)]
enum SvgError {
    Xml(quick_xml::Error),
    Svg(String),
}

// try to read image sizes from an SVG. Return an error on parsing errors or if the sizes use units
// (we don't handle those). Drop fractions.
// Will return Ok(None) if not both the `width` and the `height` attribute are present
//
// NOTES: SVGs are comples and we do not handle all cases (e.g., as mentioned above, units)
// Especially, we do not handle viewBox yet.
// As far as I understood, the conceptional difference between width/height and viewBox is:
// width/height specify the default size this SVG is rendered
// viewBox specifies the coordinates and width/height in the _internal coordinate system_ of the
// section that is rendered.
// However, they overlap a bit. At least on Firefox, I observed the following behaviour:
// - if width/height is given but not a viewBox: the viewBox will implicitly be (0, 0, width, height)
// - if viewBox is given, but not width/height: the image will fill all available space and
// maintain the viewBox aspect ratio
// - if width/height and viewBox are given, but contradict each other: the rendered section will be
// defined by the viewBox, but the rendered size will depend on the width/height, even if that will
// change the aspect ratio
// - if only one of width/height and a viewBox are given, the width or height will render as that
// and the other dimension will be determined by the viewBox (so that aspect ration is maintained)
//
// For somplification and because we control the SVG images (so we can just add width/height
// attributes), we only handle width and height attributes
fn parse_svg_size<T: BufRead>(source: T) -> Result<Option<ImageMetadata>, SvgError> {
    let mut reader = XmlReader::from_reader(source);
    let mut buf: Vec<u8> = Vec::with_capacity(256);
    // let's just assume the SVG element has to be _somewhere_ at the document's start
    // if not… well, I have control over all SVG files here, so…
    for _ in 0..5 {
        if let Event::Start(tag) = reader.read_event_into(&mut buf).map_err(SvgError::Xml)? {
            if tag.name().into_inner() == b"svg" {
                let width_str = match tag
                    .attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key.into_inner() == b"width")
                {
                    Some(a) => a,
                    None => {
                        return Ok(None);
                    }
                }
                .value;
                let height_str = match tag
                    .attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key.into_inner() == b"height")
                {
                    Some(a) => a,
                    None => {
                        return Ok(None);
                    }
                }
                .value;
                let width_str = std::str::from_utf8(&width_str)
                    .map_err(|e| SvgError::Svg(format!("invalid width encoding: {e}")))?;
                let height_str = std::str::from_utf8(&height_str)
                    .map_err(|e| SvgError::Svg(format!("invalid height encoding: {e}")))?;
                let width = parse_svg_unit(width_str).map_err(SvgError::Svg)?;
                let height = parse_svg_unit(height_str).map_err(SvgError::Svg)?;
                return Ok(Some(ImageMetadata { width, height }));
            }
        }
        buf.clear()
    }
    Err(SvgError::Svg(
        "no 'svg' tag found within first five elements".to_owned(),
    ))
}

fn parse_svg_unit(s: &str) -> Result<u32, String> {
    let s = if s.chars().all(|c| c.is_ascii_digit()) {
        s
    } else if let Some(s) = s.split_once('.').and_then(|(a, b)| {
        if b.chars().all(|c| c.is_ascii_digit()) {
            Some(a)
        } else {
            None
        }
    }) {
        s
    } else {
        return Err(format!("unknown dimension format: '{s}'"));
    };
    s.parse()
        .map_err(|e| format!("unknown dimension format: '{s}': {e}"))
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
        let img = result
            .expect("expected no error")
            .expect("expected image metadata");
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
        let img = result
            .expect("expected no error")
            .expect("expected image metadata");
        assert_eq!(img.width, 300);
        assert_eq!(img.height, 400);
    }

    #[test]
    fn read_svg_size_should_drop_fractions() {
        // given
        let svg = r##"<svg width="300.2" height="400.8" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let img = result
            .expect("expected no error")
            .expect("expected image metadata");
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
        let result = result.expect("expected no error");
        assert_eq!(result, None);
    }

    #[test]
    fn read_svg_size_should_return_none_if_height_is_missing() {
        // given
        let svg = r##"<svg width="300" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let result = result.expect("expected no error");
        assert_eq!(result, None);
    }

    #[test]
    fn read_svg_size_should_fail_if_size_has_units() {
        // given
        let svg = r##"<svg width="300px" height="400px" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let err = result.expect_err("expected error");
        if let SvgError::Svg(msg) = err {
            assert_eq!(msg, "unknown dimension format: '300px'");
        } else {
            panic!("expected svg error variant");
        }
    }

    #[test]
    fn read_svg_size_should_fail_if_size_has_units_after_decimal_point() {
        // given
        let svg = r##"<svg width="300.1px" height="400.5px" xmlns="http://www.w3.org/2000/svg"><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/></svg>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let err = result.expect_err("expected error");
        if let SvgError::Svg(msg) = err {
            assert_eq!(msg, "unknown dimension format: '300.1px'");
        } else {
            panic!("expected svg error variant");
        }
    }

    #[test]
    fn read_svg_size_should_fail_if_svg_tag_does_not_appear() {
        // given
        let svg = r##"<?xml version="1.0" encoding="UTF-8"?><path fill="red" d="M0 240h300v60H0z"/><path fill="green" d="M0 180h300v60H0z"/><path fill="#00f" d="M0 120h300v60H0z"/><path fill="purple" d="M0 60h300v60H0z"/>"##;

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let err = result.expect_err("expected error");
        if let SvgError::Svg(msg) = err {
            assert_eq!(msg, "no 'svg' tag found within first five elements");
        } else {
            panic!("expected svg error variant");
        }
    }

    #[test]
    fn read_svg_size_should_fail_for_bad_xml() {
        // given
        let svg = "<foo><bar></foo>Yo dawg I herd U liek XML";

        // when
        let result = parse_svg_size(svg.as_bytes());

        // then
        let err = result.expect_err("expected error");
        if let SvgError::Xml(e) = err {
            assert_eq!(e.to_string(), "Expecting </bar> found </foo>");
        } else {
            panic!("expected XML error variant, got {err:?}");
        }
    }

    #[test]
    fn parse_svg_unit_works_for_value_without_fraction() {
        // given
        let s = "400";

        // when
        let result = parse_svg_unit(s);

        // then
        assert_eq!(result, Ok(400));
    }

    #[test]
    fn parse_svg_unit_drops_fraction() {
        // given
        let s = "400.5";

        // when
        let result = parse_svg_unit(s);

        // then
        assert_eq!(result, Ok(400));
    }

    #[test]
    fn parse_svg_fails_if_unit_is_present() {
        // given
        let s = "400px";

        // when
        let result = parse_svg_unit(s);

        // then
        assert_eq!(result, Err("unknown dimension format: '400px'".to_owned()));
    }

    #[test]
    fn parse_svg_unit_is_present_after_fraction() {
        // given
        let s = "400.5px";

        // when
        let result = parse_svg_unit(s);

        // then
        assert_eq!(
            result,
            Err("unknown dimension format: '400.5px'".to_owned())
        );
    }
}
