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

use camino::Utf8Path;
use std::error::Error;
use std::fs::metadata;
use std::time::SystemTime;
use std::{fmt, io};

pub mod blogposts;
pub mod categories;
mod cmark;
pub mod error_pages;
pub mod feed;
mod file;
pub mod hosted_files;
mod html;
pub mod ngingx_cfg;
pub mod quotes;
pub mod tags;

/// Return true if the file referenced by filename has an older modification date than modified_at.
/// If any error occurs (e.g. the file does not exist), return true.
/// otherwise return false
fn needs_update(filename: &Utf8Path, modified_at: SystemTime) -> bool {
    metadata(filename)
        .and_then(|m| m.modified())
        .map(|t| t < modified_at)
        .unwrap_or(true)
}

/// Return true if the file reference by filename has an older modification date than any of
/// the given SystemTimes.
/// If any error occurs (e.g. the file does not exist), return true
fn needs_any_update<T>(filename: &Utf8Path, modification_times: T) -> bool
where
    T: IntoIterator<Item = SystemTime>,
{
    modification_times
        .into_iter()
        .max()
        .map(|t| needs_update(filename, t))
        .unwrap_or(true)
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RenderError {
    pub msg: String,
}

impl RenderError {
    pub fn new(msg: String) -> RenderError {
        RenderError { msg }
    }

    // only used in tests right now
    #[cfg(test)]
    pub fn from(msg: &str) -> RenderError {
        RenderError {
            msg: msg.to_owned(),
        }
    }
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub enum OutputError {
    IO(io::Error),
    Render(RenderError),
    Xml(quick_xml::Error),
}

impl Error for OutputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            OutputError::IO(e) => e,
            OutputError::Render(e) => e,
            OutputError::Xml(e) => e,
        })
    }
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputError::IO(e) => write!(f, "{e}"),
            OutputError::Render(e) => write!(f, "{e}"),
            OutputError::Xml(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for OutputError {
    fn from(e: io::Error) -> Self {
        OutputError::IO(e)
    }
}

impl From<RenderError> for OutputError {
    fn from(e: RenderError) -> Self {
        OutputError::Render(e)
    }
}

impl<T> From<io::IntoInnerError<T>> for OutputError {
    fn from(e: io::IntoInnerError<T>) -> Self {
        Self::from(io::Error::from(e))
    }
}

impl From<quick_xml::Error> for OutputError {
    fn from(e: quick_xml::Error) -> Self {
        OutputError::Xml(e)
    }
}
