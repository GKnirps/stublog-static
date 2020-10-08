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

use crate::paths::{archive_path, CATEGORIES_PATH, TAGLIST_PATH};
use chrono::{DateTime, FixedOffset};
use maud::{html, Markup, Render, DOCTYPE};

pub mod archive;
pub mod blogpost;
pub mod category;
pub mod error_pages;
pub mod home;
pub mod pager;
pub mod quote;
pub mod tag;

struct HeadData<'a> {
    title: &'a str,
    canonical_url: Option<&'a str>,
    description: Option<&'a str>,
    noindex: bool,
}

impl<'a> HeadData<'a> {
    const fn new(title: &str) -> HeadData {
        HeadData {
            title,
            canonical_url: None,
            description: None,
            noindex: false,
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
}

fn head(data: &HeadData) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title {(data.title)};
            // TODO: add cache-hash to the favicon and style filenames
            link rel="stylesheet" media="screen" href="/assets/style.css";
            link rel="icon" type="image/png" href="/assets/favicon.png";
            link rel="alternate" type="application/feed+atom" title="ATOM" href="/feed.atom";
            @if let Some(url) = data.canonical_url {
                link rel="canonical" href={(url)};
            }
            @if data.noindex {
                meta rel="robots" content="noindex, follow";
            }
            @if let Some(desc) = data.description {
                meta rel="description" content=(desc);
            }
        }
    }
}

fn base<T: Render>(head_data: &HeadData, content: T) -> Markup {
    html! {
        (DOCTYPE)
        html lang="de" {
            (head(head_data))
            body {
                div.wrap-all {
                    div.wrapper {
                        (header())
                        main.content {
                            (content)
                        }
                        (footer())
                    }
                }
            }
        }
    }
}

fn header() -> Markup {
    html! {
        header {
            div.banner {
                h1 {
                    a href="/" {"Stranger Than Usual"}
                }
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
                    a href="https://creativecommons.org/licenses/by-nc-sa/4.0/" {
                        "Creative Commons BY-NC-SA 4.0"
                    }"."
            }
            p.privacy {
                "Datenschutz: Diese Website erhebt keine Benutzerdaten. HTTP-Requestdaten (insbes. IP-Adress und user agent) bleiben nur für die Dauer des Requests bestehen."
            }
        }
    }
}

fn time(t: &DateTime<FixedOffset>) -> Markup {
    let iso_time = t.to_rfc3339();
    html! {
        time datetime=(iso_time) {
            (t.format("%d.%m.%Y %H:%M"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn head_should_render_everything_given() {
        // given
        let head_data = HeadData::new("IM IN UR TITLE")
            .with_canonical_url("https::/example.com/canon")
            .with_description(Some("This is a sample page\""))
            .with_noindex();

        // when
        let result = head(&head_data).into_string();

        // then
        println!("Checking html:\n{}", result);

        // fields that depend on head_data
        assert!(result.contains("<title>IM IN UR TITLE</title>"));
        assert!(
            result.contains("<meta rel=\"description\" content=\"This is a sample page&quot;\">")
        );
        assert!(result.contains("<link rel=\"canonical\" href=\"https::/example.com/canon\">"));
        assert!(result.contains("<meta rel=\"robots\" content=\"noindex, follow\">"));

        // static fields
        assert!(result.starts_with("<head><meta charset=\"utf-8\">"));
        assert!(result
            .contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"));
        assert!(result
            .contains("<link rel=\"stylesheet\" media=\"screen\" href=\"/assets/style.css\">"));
        assert!(
            result.contains("<link rel=\"icon\" type=\"image/png\" href=\"/assets/favicon.png\">")
        );
        assert!(result.contains("<link rel=\"alternate\" type=\"application/feed+atom\" title=\"ATOM\" href=\"/feed.atom\">"));
    }

    #[test]
    fn head_should_render_with_minimal_data() {
        // given
        let head_data = HeadData::new("IM IN UR TITLE");

        // when
        let result = head(&head_data).into_string();

        // then
        println!("Checking html:\n{}", result);

        // fields that depend on head_data
        assert!(result.contains("<title>IM IN UR TITLE</title>"));
        assert!(!result.contains("<link rel=\"canonical\""));
        assert!(!result.contains("<meta rel=\"robots\""));
        assert!(!result.contains("<meta rel=\"description\""));

        // static fields
        assert!(result.starts_with("<head><meta charset=\"utf-8\">"));
        assert!(result
            .contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"));
        assert!(result
            .contains("<link rel=\"stylesheet\" media=\"screen\" href=\"/assets/style.css\">"));
        assert!(
            result.contains("<link rel=\"icon\" type=\"image/png\" href=\"/assets/favicon.png\">")
        );
        assert!(result.contains("<link rel=\"alternate\" type=\"application/feed+atom\" title=\"ATOM\" href=\"/feed.atom\">"));
    }

    #[test]
    fn base_should_render_header_fields_and_content() {
        // given
        let head_data =
            HeadData::new("There will be cake").with_canonical_url("https://example.com/foo");
        let content = "The cake is a lie";

        // when
        let result = base(&head_data, content).into_string();

        // then
        println!("Checking headers of {}", result);
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
    }

    #[test]
    fn base_should_not_render_canonical_url_if_not_present() {
        // given
        let head_data = HeadData::new("There will be cake");
        let content = "The cake is a lie";

        // when
        let result = base(&head_data, content).into_string();

        // then
        assert!(!result.contains("rel=\"canonical\""));
    }

    #[test]
    fn time_should_render_valid_time_tag() {
        // given
        let date = FixedOffset::east(3600 * 2)
            .ymd(2020, 5, 11)
            .and_hms(12, 13, 14);

        // when
        let result = time(&date).into_string();

        // then
        assert_eq!(
            result,
            "<time datetime=\"2020-05-11T12:13:14+02:00\">11.05.2020 12:13</time>"
        );
    }
}
