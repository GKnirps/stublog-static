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

use maud::{html, Markup, Render};

type PathGenerator = dyn Fn(usize) -> String;

fn page_link<T: Render>(
    index: usize,
    content: T,
    label: &str,
    class: Option<&str>,
    generate_path: &PathGenerator,
) -> Markup {
    let path = generate_path(index);
    // sigh… right now there does not seem to be a good way to completely omit an attribute… in
    // maud.
    match class {
        Some(c) => html! {
            a class=(c) aria-label=(label) href=(path) title=(label) { (content) }
        },
        None => html! {
            a aria-label=(label) href=(path) title=(label) { (content) }
        },
    }
}

fn numbered_page_link(index: usize, current_page: usize, generate_path: &PathGenerator) -> Markup {
    let disp_index = index + 1;
    let label = format!("Seite {}", disp_index);

    if index == current_page {
        html! {
            span.current aria-label=(label) title=(label) {
                (disp_index)
            }
        }
    } else {
        page_link(index, disp_index, &label, None, generate_path)
    }
}

fn prev_link(current_page: usize, generate_path: &PathGenerator) -> Markup {
    let content = "← zurück";
    let class = "previous-page";
    if current_page == 0 {
        html! {
            span class=(class) {(content)}
        }
    } else {
        page_link(
            current_page - 1,
            content,
            "zurückblättern",
            Some(class),
            generate_path,
        )
    }
}

fn next_link(current_page: usize, num_pages: usize, generate_path: &PathGenerator) -> Markup {
    let content = "vorwärts →";
    let class = "next-page";
    if current_page + 1 == num_pages {
        html! {
            span class="next-page" {(content)}
        }
    } else {
        page_link(
            current_page + 1,
            content,
            "weiterblättern",
            Some(class),
            generate_path,
        )
    }
}

pub fn pager(page_index: usize, num_pages: usize, generate_path: &PathGenerator) -> Markup {
    if num_pages < 2 {
        return html! {};
    }

    // the previous blog implementation seemed to show one link for each page no
    // matter how many pages there are
    // For simplicity, just leave it that way for now.
    // This may result in an excessive amount of page links, so we may need to
    // change this at some time.

    html! {
        nav.pagination {
            ul {
                li { (prev_link(page_index, generate_path)) }
                @for i in 0..num_pages {
                    li { (numbered_page_link(i, page_index, generate_path)) }
                }
                li { (next_link(page_index, num_pages, generate_path)) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_path(i: usize) -> String {
        format!("/dummy/{}", i)
    }

    #[test]
    fn page_link_renders_valid_link_with_class() {
        // given
        let index = 42;
        let content = "somewhere";
        let label = "click!";
        let class = Some("classy");

        // when
        let result = page_link(index, content, label, class, &dummy_path).into_string();

        // then
        assert_eq!(
            &result,
            "<a class=\"classy\" aria-label=\"click!\" href=\"/dummy/42\" title=\"click!\">somewhere</a>"
        );
    }

    #[test]
    fn page_link_renders_valid_link_without_class() {
        // given
        let index = 42;
        let content = "somewhere";
        let label = "click!";
        let class = None;

        // when
        let result = page_link(index, content, label, class, &dummy_path).into_string();

        // then
        assert_eq!(
            &result,
            "<a aria-label=\"click!\" href=\"/dummy/42\" title=\"click!\">somewhere</a>"
        );
    }

    #[test]
    fn numbered_page_link_renders_valid_link() {
        // given
        let index = 42;
        let current_page = 0;

        // when
        let result = numbered_page_link(index, current_page, &dummy_path).into_string();

        // then
        assert_eq!(
            &result,
            "<a aria-label=\"Seite 43\" href=\"/dummy/42\" title=\"Seite 43\">43</a>"
        );
    }

    #[test]
    fn numbered_page_link_renders_no_link_for_current_page() {
        // given
        let index = 42;
        let current_page = 42;

        // when
        let result = numbered_page_link(index, current_page, &dummy_path).into_string();

        // then
        assert_eq!(
            &result,
            "<span class=\"current\" aria-label=\"Seite 43\" title=\"Seite 43\">43</span>"
        );
    }

    #[test]
    fn prev_link_should_render_content_without_link_for_first_page() {
        // given
        let index = 0;

        // when
        let result = prev_link(index, &dummy_path).into_string();

        // then
        assert_eq!(&result, "<span class=\"previous-page\">← zurück</span>");
    }

    #[test]
    fn prev_link_should_render_content_with_link_for_further_pages() {
        // given
        let index = 42;

        // when
        let result = prev_link(index, &dummy_path).into_string();

        // then
        assert_eq!(&result, "<a class=\"previous-page\" aria-label=\"zurückblättern\" href=\"/dummy/41\" title=\"zurückblättern\">← zurück</a>");
    }

    #[test]
    fn next_link_should_render_content_without_link_for_last_page() {
        // given
        let index = 42;
        let num_pages = 43;

        // when
        let result = next_link(index, num_pages, &dummy_path).into_string();

        // then
        assert_eq!(&result, "<span class=\"next-page\">vorwärts →</span>");
    }

    #[test]
    fn next_link_should_render_content_with_link_for_earlier_pages() {
        // given
        let index = 42;
        let num_pages = 9001;

        // when
        let result = next_link(index, num_pages, &dummy_path).into_string();

        // then
        assert_eq!(&result, "<a class=\"next-page\" aria-label=\"weiterblättern\" href=\"/dummy/43\" title=\"weiterblättern\">vorwärts →</a>");
    }

    #[test]
    fn pager_renders_nothing_for_only_one_page() {
        // given
        let page_index = 0;
        let num_pages = 1;

        // when
        let result = pager(page_index, num_pages, &dummy_path).into_string();

        // then
        assert_eq!(&result, "");
    }

    #[test]
    fn pager_renders_links_to_all_pages() {
        // given
        let page_index = 1;
        let num_pages = 3;

        // when
        let result = pager(page_index, num_pages, &dummy_path).into_string();

        // then
        assert_eq!(&result, "<nav class=\"pagination\"><ul>\
        <li><a class=\"previous-page\" aria-label=\"zurückblättern\" href=\"/dummy/0\" title=\"zurückblättern\">← zurück</a></li>\
        <li><a aria-label=\"Seite 1\" href=\"/dummy/0\" title=\"Seite 1\">1</a></li>\
        <li><span class=\"current\" aria-label=\"Seite 2\" title=\"Seite 2\">2</span></li>\
        <li><a aria-label=\"Seite 3\" href=\"/dummy/2\" title=\"Seite 3\">3</a></li>\
        <li><a class=\"next-page\" aria-label=\"weiterblättern\" href=\"/dummy/2\" title=\"weiterblättern\">vorwärts →</a></li>\
        </ul></nav>");
    }
}
