use chrono::{DateTime, FixedOffset};
use maud::{html, Markup, Render, DOCTYPE};

pub mod blogpost;

fn base<T: Render>(title: &str, content: T) -> Markup {
    html! {
        (DOCTYPE)
        html lang="de" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title {"Stranger Than Usual — " (title)};
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
                    li { a href="/" {"Home"} }
                    li { a href="#" {"Archiv"} }
                    li { a href="#" {"Kategorien"} }
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

        // when
        let result = base(title, content).into_string();

        // then
        println!("Checking headers of {}", result);
        assert!(result.contains("<meta charset=\"utf-8\">"));
        assert!(result
            .contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"));
        assert!(result.contains("<title>There will be cake</title>"));
        assert!(result.contains(content));
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
