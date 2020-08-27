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

    super::base("Stranger Than Usual — Seite nicht gefunden", content)
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
