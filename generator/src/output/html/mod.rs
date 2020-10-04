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

fn base<T: Render>(title: &str, content: T, canonical_url: Option<&str>) -> Markup {
    html! {
        (DOCTYPE)
        html lang="de" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title {(title)};
                // TODO: add cache-hash to the favicon and style filenames
                link rel="stylesheet" media="screen" href="/assets/style.css";
                link rel="icon" type="image/png" href="/assets/favicon.png";
                link rel="alternate" type="application/feed+atom" title="ATOM" href="/feed.atom";
                @if let Some(url) = canonical_url {
                    link rel="canonical" href={(url)};
                }
            }
            body {
                div.wrap-all {
                    div.wrapper {
                        (header())
                        main.content {
                            (content)
                        }
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
    fn base_should_render_header_fields_and_content() {
        // given
        let title = "There will be cake";
        let content = "The cake is a lie";
        let canonical_url = Some("https://example.com/foo");

        // when
        let result = base(title, content, canonical_url).into_string();

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
        let title = "There will be cake";
        let content = "The cake is a lie";
        let canonical_url = None;

        // when
        let result = base(title, content, canonical_url).into_string();

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
