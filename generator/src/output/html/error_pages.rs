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

use crate::output::html::HeadData;
use maud::{html, Markup};

pub fn render_404() -> Markup {
    let content = html! {
        div.error {
            h2 { "404 — nicht gefunden" }
            p {
                "Was auch immer du gesucht hast, hier ist es nicht. Es sei denn, du hast eine Fehlerseite gesucht. Dann bist du hier genau richtig."
            }
        }
    };

    super::base(
        &HeadData::new("Stranger Than Usual — Seite nicht gefunden"),
        content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_404_renders_error_page() {
        // when
        let result = render_404().into_string();

        // then
        println!("Checking HTML: {}\n", result);
        assert!(result.contains("<div class=\"error\">"));
        assert!(result.starts_with("<!DOCTYPE html>"));
    }
}
