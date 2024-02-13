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

use crate::input::Assets;
use crate::paths::{archive_path, CATEGORIES_PATH, TAGLIST_PATH};
use chrono::{DateTime, FixedOffset};
use maud::{html, Markup, Render, DOCTYPE};

pub mod archive;
pub mod blogpost;
pub mod category;
pub mod error_pages;
pub mod home;
pub mod hosted_file;
pub mod pager;
pub mod quote;
pub mod tag;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct HeadData<'a> {
    title: &'a str,
    canonical_url: Option<&'a str>,
    description: Option<&'a str>,
    noindex: bool,
    og_type: Option<&'a str>,
    og_image_url: Option<&'a str>,
    assets: &'a Assets,
}

impl<'a> HeadData<'a> {
    const fn new(title: &'a str, assets: &'a Assets) -> HeadData<'a> {
        HeadData {
            title,
            canonical_url: None,
            description: None,
            noindex: false,
            og_type: None,
            og_image_url: None,
            assets,
        }
    }

    const fn with_canonical_url(mut self, canonical_url: &'a str) -> HeadData<'a> {
        self.canonical_url = Some(canonical_url);
        self
    }

    const fn with_description(mut self, description: Option<&'a str>) -> HeadData<'a> {
        self.description = description;
        self
    }

    const fn with_noindex(mut self) -> HeadData<'a> {
        self.noindex = true;
        self
    }

    const fn with_og_type(mut self, og_type: &'a str) -> HeadData<'a> {
        self.og_type = Some(og_type);
        self
    }

    const fn with_og_image_url(mut self, url: &'a str) -> HeadData<'a> {
        self.og_image_url = Some(url);
        self
    }
}

fn head(data: &HeadData) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title {(data.title)};
            link rel="stylesheet" media="screen" href=(data.assets.stylesheet.web_path);
            link rel="icon" type="image/png" href=(data.assets.favicon.web_path);
            link rel="alternate" type="application/feed+atom" title="ATOM" href="/feed.atom";
            @if let Some(url) = data.canonical_url {
                link rel="canonical" href={(url)};
            }
            @if data.noindex {
                meta name="robots" content="noindex, follow";
            }
            @if let Some(desc) = data.description {
                meta name="description" content=(desc);
            }
            (opengraph_tags(data))
        }
    }
}

fn opengraph_tag(name: &str, content: Option<&str>) -> Markup {
    html! {
        @if let Some(c) = content {
            meta property=(name) content=(c);
        }
    }
}

fn opengraph_tags(data: &HeadData) -> Markup {
    html! {
        (opengraph_tag("og:title", Some(data.title)))
        (opengraph_tag("og:url", data.canonical_url))
        (opengraph_tag("og:type", data.og_type))
        (opengraph_tag("og:description", data.description))
        (opengraph_tag("og:locale", Some("de_DE")))
        (opengraph_tag("og:site_name", Some("Stranger Than Usual")))
        (opengraph_tag("og:image", data.og_image_url))
    }
}

fn base<T: Render>(head_data: &HeadData, content: T) -> Markup {
    html! {
        (DOCTYPE)
        html lang="de" {
            (head(head_data))
            body {
                (header())
                main.content {
                    (content)
                }
                (footer())
            }
        }
        "\n"
    }
}

fn header() -> Markup {
    html! {
        header.header-global {
            h1 {
                a href="/" {"Stranger Than Usual"}
            }
            nav.navigation {
                ul.main-nav {
                    li { a href=(archive_path(0)) {"Archiv"} }
                    li { a href=(CATEGORIES_PATH) {"Kategorien"} }
                    li { a href=(TAGLIST_PATH) {"Tags"} }
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer.footer-global {
            p.license {
                "Die von mir verfassten Inhalte stehen, soweit nicht anders vermerkt, unter der Lizenz "
                    a href="https://creativecommons.org/licenses/by-nc-sa/4.0/" rel="license" {
                        "Creative Commons BY-NC-SA 4.0"
                    }"."
            }
            p.license-generator {
                "Dieses Blog wurde mit "
                a href="https://github.com/GKnirps/stublog-static" {
                    "stublog-static"
                }
                " generiert."
            }
            p.privacy {
                "Datenschutz: Diese Website erhebt keine Benutzerdaten. HTTP-Requestdaten (insbes. IP-Adresse und user agent) bleiben nur für die Dauer des Requests bestehen."
            }
        }
    }
}

fn time(t: &DateTime<FixedOffset>) -> Markup {
    let iso_time = t.to_rfc3339();
    html! {
        time datetime=(iso_time) {
            (maud::display(t.format("%d.%m.%Y %H:%M")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_assets;
    use chrono::TimeZone;

    #[test]
    fn head_should_render_everything_given() {
        // given
        let mut assets = create_assets();
        assets.stylesheet.web_path = "/assets/style.css?cache=deadbeef".to_owned();
        assets.favicon.web_path = "/assets/favicon.png?cache=cafebabe".to_owned();
        let head_data = HeadData::new("IM IN UR TITLE", &assets)
            .with_canonical_url("https://example.com/canon")
            .with_description(Some("This is a sample page\""))
            .with_noindex();

        // when
        let result = head(&head_data).into_string();

        // then
        println!("Checking html:\n{result}");

        // fields that depend on head_data
        assert!(result.contains("<title>IM IN UR TITLE</title>"));
        assert!(
            result.contains("<meta name=\"description\" content=\"This is a sample page&quot;\">")
        );
        assert!(result.contains("<link rel=\"canonical\" href=\"https://example.com/canon\">"));
        assert!(result.contains("<meta name=\"robots\" content=\"noindex, follow\">"));

        // static fields
        assert!(result.starts_with("<head><meta charset=\"utf-8\">"));
        assert!(result
            .contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"));
        assert!(result.contains(
            "<link rel=\"stylesheet\" media=\"screen\" href=\"/assets/style.css?cache=deadbeef\">"
        ));
        assert!(result.contains(
            "<link rel=\"icon\" type=\"image/png\" href=\"/assets/favicon.png?cache=cafebabe\">"
        ));
        assert!(result.contains("<link rel=\"alternate\" type=\"application/feed+atom\" title=\"ATOM\" href=\"/feed.atom\">"));
    }

    #[test]
    fn head_should_render_with_minimal_data() {
        // given
        let mut assets = create_assets();
        assets.stylesheet.web_path = "/assets/style.css?cache=deadbeef".to_owned();
        assets.favicon.web_path = "/assets/favicon.png?cache=cafebabe".to_owned();

        let head_data = HeadData::new("IM IN UR TITLE", &assets);

        // when
        let result = head(&head_data).into_string();

        // then
        println!("Checking html:\n{result}");

        // fields that depend on head_data
        assert!(result.contains("<title>IM IN UR TITLE</title>"));
        assert!(!result.contains("<link rel=\"canonical\""));
        assert!(!result.contains("<meta name=\"robots\""));
        assert!(!result.contains("<meta name=\"description\""));

        // static fields
        assert!(result.starts_with("<head><meta charset=\"utf-8\">"));
        assert!(result
            .contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"));
        assert!(result.contains(
            "<link rel=\"stylesheet\" media=\"screen\" href=\"/assets/style.css?cache=deadbeef\">"
        ));
        assert!(result.contains(
            "<link rel=\"icon\" type=\"image/png\" href=\"/assets/favicon.png?cache=cafebabe\">"
        ));
        assert!(result.contains("<link rel=\"alternate\" type=\"application/feed+atom\" title=\"ATOM\" href=\"/feed.atom\">"));

        // just a short check if any opengraph tags are rendered
        assert!(result.contains("<meta property=\"og:site_name\" content=\"Stranger Than Usual\">"));
    }

    #[test]
    fn opengraph_tags_should_render_all_tags_if_all_data_is_given() {
        // given
        let assets = create_assets();
        let head_data = HeadData::new("I blew up the moon", &assets)
            .with_og_type("mad-science")
            .with_description(Some("I wasn't planning it, but it was the best solution."))
            .with_canonical_url("https://example.com/moon")
            .with_og_image_url("https://imgs.xkcd.com/comics/recipes.png");

        // when
        let result = opengraph_tags(&head_data).into_string();

        // then
        assert_eq!(
            result,
            "<meta property=\"og:title\" content=\"I blew up the moon\">\
            <meta property=\"og:url\" content=\"https://example.com/moon\">\
            <meta property=\"og:type\" content=\"mad-science\">\
            <meta property=\"og:description\" content=\"I wasn\'t planning it, but it was the best solution.\">\
            <meta property=\"og:locale\" content=\"de_DE\">\
            <meta property=\"og:site_name\" content=\"Stranger Than Usual\">\
            <meta property=\"og:image\" content=\"https://imgs.xkcd.com/comics/recipes.png\">"
        );
    }

    #[test]
    fn opengraph_tags_should_render_minimal_tags() {
        // given
        let assets = create_assets();
        let head_data = HeadData::new("I blew up the moon", &assets);

        // when
        let result = opengraph_tags(&head_data).into_string();

        // then
        assert_eq!(
            result,
            "<meta property=\"og:title\" content=\"I blew up the moon\">\
            <meta property=\"og:locale\" content=\"de_DE\">\
            <meta property=\"og:site_name\" content=\"Stranger Than Usual\">"
        );
    }

    #[test]
    fn base_should_render_header_fields_and_content() {
        // given
        let assets = create_assets();
        let head_data = HeadData::new("There will be cake", &assets)
            .with_canonical_url("https://example.com/foo");
        let content = "The cake is a lie";

        // when
        let result = base(&head_data, content).into_string();

        // then
        println!("Checking headers of {result}");
        assert!(result.contains("<meta charset=\"utf-8\">"));
        assert!(result
            .contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"));
        assert!(result.contains("<link rel=\"canonical\" href=\"https://example.com/foo\">"));
        assert!(result.contains("<title>There will be cake</title>"));
        assert!(
            result.contains("<a href=\"/archive/0\">Archiv</a>"),
            "Expected archive link to be on the page"
        );
        assert!(result.contains(content));
        assert_eq!(result.chars().next_back(), Some('\n'));
    }

    #[test]
    fn base_should_not_render_canonical_url_if_not_present() {
        // given
        let assets = create_assets();
        let head_data = HeadData::new("There will be cake", &assets);
        let content = "The cake is a lie";

        // when
        let result = base(&head_data, content).into_string();

        // then
        assert!(!result.contains("rel=\"canonical\""));
    }

    #[test]
    fn time_should_render_valid_time_tag() {
        // given
        let date = FixedOffset::east_opt(3600 * 2)
            .unwrap()
            .with_ymd_and_hms(2020, 5, 11, 12, 13, 14)
            .unwrap();

        // when
        let result = time(&date).into_string();

        // then
        assert_eq!(
            result,
            "<time datetime=\"2020-05-11T12:13:14+02:00\">11.05.2020 12:13</time>"
        );
    }
}
