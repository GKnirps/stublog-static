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

use super::super::cmark::render_blogpost_content;
use crate::HostedFile;
use crate::input::{Assets, Blogpost, Category};
use crate::output::html::{HeadData, OgImageData};
use crate::output::{RenderError, image_metadata_by_path};
use crate::paths::{blogpost_path, category_path, tag_path};
use crate::urls::{blogpost_url, url_for_absolute_path};
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;

pub fn render_blogpost(
    blogpost: &Blogpost,
    category: &Category,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let permalink = blogpost_path(blogpost);
    Ok(html! {
        article.blogpost lang=[blogpost.language] {
            h2.posttitle {
                a href=(permalink) rel="bookmark" {
                    (blogpost.title)
                }
            }
            div.entry { (PreEscaped(render_blogpost_content(blogpost, hosted_files)?)) }
            footer {
                div.post-time {
                    (super::time(&blogpost.date))
                }
                @if let Some(update_date) = &blogpost.update_date {
                    div.update-time {
                        "Zuletzt geändert: " (super::time(update_date))
                    }
                }
                div.category {
                    "Kategorie: "
                    a href=(category_path(category)) { (category.title) }
                }
                @if !blogpost.tags.is_empty() {
                    div.tags {
                        "Tags: "
                        ul {
                            @for tag in &blogpost.tags {
                                li {
                                    a href=(tag_path(tag)) {
                                        (tag.name)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

/// render a list of blogposts with just title, description and creation time
/// meant to be used for overviews of blogposts (e.g. on category or tag pages)
pub fn render_blogpost_summary_list(blogposts: &[&Blogpost]) -> Markup {
    html!(
        ul {
            @for blogpost in blogposts {
                li lang = [blogpost.language]{
                    h4.posttitle {
                        a href=(blogpost_path(blogpost)) {
                            (blogpost.title)
                        }
                        " (" (super::time(&blogpost.date)) ")"
                    }
                    p { (blogpost.summary) }
                }
            }
        }
    )
}

pub fn render_blogpost_page(
    blogpost: &Blogpost,
    category: &Category,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let description: Option<&str> = Some(&blogpost.summary);
    let og_image = create_og_image_data(blogpost, hosted_files)?;

    let post_url = blogpost_url(blogpost);
    let mut head_data = HeadData::new(&blogpost.title, assets)
        .with_canonical_url(&post_url)
        .with_description(description)
        .with_og_type("article");
    if let Some(image) = og_image {
        head_data = head_data.with_og_image(image)
    }

    Ok(super::base(
        &head_data,
        render_blogpost(blogpost, category, hosted_files)?,
    ))
}

fn create_og_image_data<'a>(
    blogpost: &'a Blogpost,
    hosted_files: &HashMap<&str, &'a HostedFile>,
) -> Result<Option<OgImageData<'a>>, RenderError> {
    if let Some(image) = &blogpost.image {
        let url = url_for_absolute_path(&image.path);
        let alt = &image.alt;
        let metadata = image_metadata_by_path(&image.path, hosted_files)?;
        Ok(Some(OgImageData { url, alt, metadata }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{ImageMetadata, Language, OgImage};
    use crate::test_utils::{create_assets, create_blogpost, create_category, create_hosted_file};

    #[test]
    fn render_blogpost_should_render_blogpost_with_footer() {
        // given
        let blogpost = create_blogpost();
        let category = create_category();
        let hosted_files = HashMap::new();

        // when
        let result = render_blogpost(&blogpost, &category, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(result.contains(r#"<article class="blogpost">"#));
        assert!(result
            .contains("<h2 class=\"posttitle\"><a href=\"/blogposts/foobar\" rel=\"bookmark\">Nevermind</a></h2>"));
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p>\n</div>"));
        assert!(result.contains("11.05.2020 12:13"));
        assert!(result.contains(r#"<div class="update-time">Zuletzt geändert: <time datetime="2020-05-25T12:13:14+02:00">25.05.2020 12:13</time></div>"#));
        assert!(result.contains("<a href=\"/categories/chocolate\">Cocoa</a>"));
        assert!(result.contains(
            r#"<div class="tags">Tags: <ul><li><a href="/tags/foo">foo</a></li><li><a href="/tags/bar">bar</a></li></ul>"#
        ));
    }

    #[test]
    fn render_blogpost_does_not_render_tags_if_tag_list_is_empty() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.tags = Vec::new();
        let category = create_category();
        let hosted_files = HashMap::new();

        // when
        let result = render_blogpost(&blogpost, &category, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(!result.contains(r#"class="tags""#));
    }

    #[test]
    fn render_blogpost_does_not_render_update_time_if_not_present() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.update_date = None;
        let category = create_category();
        let hosted_files = HashMap::new();

        // when
        let result = render_blogpost(&blogpost, &category, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(!result.contains(r#"class="update-time""#));
    }

    #[test]
    fn render_blogpost_should_render_language_if_present() {
        // given
        let category = create_category();
        let mut blogpost = create_blogpost();
        blogpost.language = Some(Language::En);
        let hosted_files = HashMap::new();

        // when
        let result = render_blogpost(&blogpost, &category, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(result.contains(r#"<article class="blogpost" lang="en">"#));
    }

    #[test]
    fn render_blogpost_summary_list_renders_title_date_and_summary() {
        // given
        let mut blogpost1 = create_blogpost();
        blogpost1.title = "Teil 1: Deutsch".to_string();
        blogpost1.summary = "Irgendwas auf Deutsch".to_string();

        let mut blogpost2 = create_blogpost();
        blogpost2.title = "Part 2: English".to_string();
        blogpost2.summary = "Something in English".to_string();
        blogpost2.language = Some(Language::En);

        let blogposts = &[&blogpost1, &blogpost2];

        // when
        let html = render_blogpost_summary_list(blogposts);

        // then
        assert_eq!(
            html.into_string(),
            "<ul><li><h4 class=\"posttitle\"><a href=\"/blogposts/foobar\">Teil 1: Deutsch</a> (<time datetime=\"2020-05-11T12:13:14+02:00\">11.05.2020 12:13</time>)</h4><p>Irgendwas auf Deutsch</p></li><li lang=\"en\"><h4 class=\"posttitle\"><a href=\"/blogposts/foobar\">Part 2: English</a> (<time datetime=\"2020-05-11T12:13:14+02:00\">11.05.2020 12:13</time>)</h4><p>Something in English</p></li></ul>"
        );
    }

    #[test]
    fn render_blogpost_page_should_render_blogpost() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.image = None;
        let category = create_category();

        let assets = create_assets();

        let hosted_files = HashMap::new();

        // when
        let result = render_blogpost_page(&blogpost, &category, &assets, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(result.contains("<meta name=\"description\" content=\"foo!\">"));
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p>\n</div>"));
        assert!(result.contains("<title>Nevermind</title>"));
        assert!(result.contains("11.05.2020 12:13"));
        assert!(result.contains("<a href=\"/categories/chocolate\">Cocoa</a>"));
    }

    #[test]
    fn create_og_image_data_returns_image_with_all_metadata_if_present() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.image = Some(OgImage {
            path: "/file/answer.png".to_owned(),
            alt: "fourty-two".to_owned(),
        });
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let hosted_files: HashMap<&str, &HostedFile> =
            HashMap::from([("answer.png", &hosted_file)]);

        // when
        let result = create_og_image_data(&blogpost, &hosted_files);

        // then
        let metadata = result.expect("expected success");
        assert_eq!(
            metadata,
            Some(OgImageData {
                url: "https://blog.strangerthanusual.de/file/answer.png".to_owned(),
                alt: "fourty-two",
                metadata: Some(&ImageMetadata {
                    width: 42,
                    height: 9001
                })
            })
        );
    }

    #[test]
    fn create_og_image_data_returns_empty_data_for_absent_image() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.image = None;
        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let result = create_og_image_data(&blogpost, &hosted_files);

        // then
        let metadata = result.expect("expected success");
        assert_eq!(metadata, None);
    }

    #[test]
    fn create_og_image_data_fails_for_missing_image_file() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.image = Some(OgImage {
            path: "/file/answer.png".to_owned(),
            alt: "fourty-two".to_owned(),
        });
        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let result = create_og_image_data(&blogpost, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from("did not find hosted image 'answer.png'"))
        );
    }
}
