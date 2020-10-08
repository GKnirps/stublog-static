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

use super::file::open_for_write;
use crate::input::{Category, HostedFile};
use crate::paths;
use std::io::Write;
use std::path::Path;

/// write an ngingx rule to make a file that has been imported from the old blog
/// accessible under the same URL as before
fn write_hosted_file_rewrite(w: &mut dyn Write, hosted_file: &HostedFile) -> std::io::Result<()> {
    if let Some(ref old_id) = hosted_file.old_id {
        let new_path = paths::hosted_file_path(hosted_file);
        // TODO: we assume the id and the new path do not contain any dangerous stuff.
        //  maybe we need to change that. On the other hand, this config generator is only supposed
        //  to be run to create the config for imported data from the old blog, this is not supposed
        //  to be done after the start of this blog. Until then, the generated config files need to
        //  be checked manually

        // we make an internal rewrite, so the file is available via two different URLs.
        // this way, users who access old blogposts will not get a redirect for each and every image
        writeln!(
            w,
            "rewrite ^/hosted_files/{}/download$ {};",
            old_id, new_path
        )
    } else {
        Ok(())
    }
}

fn write_hosted_files_rewrites(
    w: &mut dyn Write,
    hosted_files: &[HostedFile],
) -> std::io::Result<()> {
    writeln!(w, "## old paths for hosted files")?;
    for hosted_file in hosted_files {
        write_hosted_file_rewrite(w, hosted_file)?;
    }
    Ok(())
}

/// write an ngingx rule to make a blog category that was imported from the old blog accessible via
/// its old URL
fn write_category_rewrite(w: &mut dyn Write, category: &Category) -> std::io::Result<()> {
    if let Some(ref old_id) = category.old_id {
        let new_path = paths::category_path(category);
        // TODO: we assume the id and the new path do not contain any dangerous stuff.
        //  maybe we need to change that. On the other hand, this config generator is only supposed
        //  to be run to create the config for imported data from the old blog, this is not supposed
        //  to be done after the start of this blog. Until then, the generated config files need to
        //  be checked manually

        // we make an internal rewrite, so the file is available via two different URLs.
        // this way, users who access old blogposts will not get a redirect for each and every image
        writeln!(w, "rewrite ^/categories/{}$ {};", old_id, new_path)
    } else {
        Ok(())
    }
}

fn write_categories_rewrites(w: &mut dyn Write, categories: &[Category]) -> std::io::Result<()> {
    writeln!(w, "## old paths for categories")?;
    for category in categories {
        write_category_rewrite(w, category)?;
    }
    Ok(())
}

pub fn write_config_file(
    dir: &Path,
    categories: &[Category],
    hosted_files: &[HostedFile],
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push("old_paths_rewrites.conf");

    let mut writer = open_for_write(&filename)?;

    write_categories_rewrites(&mut writer, categories)?;

    writeln!(writer)?;

    write_hosted_files_rewrites(&mut writer, hosted_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_category, create_hosted_file};

    #[test]
    fn write_hosted_file_rewrite_writes_valid_rewrite_for_old_id() {
        // given
        let mut hosted_file = create_hosted_file();
        hosted_file.old_id = Some("42".to_owned());
        hosted_file.path = "answer.txt".to_owned();

        // when
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        write_hosted_file_rewrite(&mut buffer, &hosted_file).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8 in output");

        // then
        assert_eq!(
            result,
            "rewrite ^/hosted_files/42/download$ /file/answer.txt;\n"
        );
    }

    #[test]
    fn write_hosted_file_rewrite_writes_nothing_if_no_old_id_is_present() {
        // given
        let mut hosted_file = create_hosted_file();
        hosted_file.old_id = None;

        // when
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        write_hosted_file_rewrite(&mut buffer, &hosted_file).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8 in output");

        // then
        assert_eq!(result, "");
    }

    #[test]
    fn write_hosted_files_rewrites_writes_list_of_rules() {
        // given
        let mut f1 = create_hosted_file();
        f1.old_id = Some("11".to_owned());
        f1.path = "spinal.tap".to_owned();

        let mut f2 = create_hosted_file();
        f2.old_id = None;

        let mut f3 = create_hosted_file();
        f3.old_id = Some("9001".to_owned());
        f3.path = "its.over".to_owned();

        let files = &[f1, f2, f3];

        // when
        let mut buffer: Vec<u8> = Vec::with_capacity(200);
        write_hosted_files_rewrites(&mut buffer, files).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8 in output");

        // then
        assert_eq!(
            result,
            "## old paths for hosted files\n\
            rewrite ^/hosted_files/11/download$ /file/spinal.tap;\n\
            rewrite ^/hosted_files/9001/download$ /file/its.over;\n"
        );
    }

    #[test]
    fn write_category_rewrite_writes_valid_rewrite_for_old_id() {
        // given
        let mut cat = create_category();
        cat.old_id = Some("11".to_owned());
        cat.filename = Path::new("spinal").to_path_buf();

        // when
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        write_category_rewrite(&mut buffer, &cat).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8 in output");

        // then
        assert_eq!(result, "rewrite ^/categories/11$ /categories/spinal;\n");
    }

    #[test]
    fn write_category_rewrite_writes_nothing() {
        // given
        let mut cat = create_category();
        cat.old_id = None;

        // when
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        write_category_rewrite(&mut buffer, &cat).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8 in output");

        // then
        assert_eq!(result, "");
    }

    #[test]
    fn write_categories_rewrites_writes_list_of_rules() {
        // given
        let mut cat1 = create_category();
        cat1.old_id = Some("11".to_owned());
        cat1.filename = Path::new("spinal").to_path_buf();

        let mut cat2 = create_category();
        cat2.old_id = None;

        let mut cat3 = create_category();
        cat3.old_id = Some("42".to_owned());
        cat3.filename = Path::new("answers").to_path_buf();

        let cats = &[cat1, cat2, cat3];

        // when
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        write_categories_rewrites(&mut buffer, cats).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8 in output");

        // then
        assert_eq!(
            result,
            "## old paths for categories\n\
            rewrite ^/categories/11$ /categories/spinal;\n\
            rewrite ^/categories/42$ /categories/answers;\n"
        );
    }
}
