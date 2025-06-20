use maud::{html, Markup, DOCTYPE};

pub(crate) fn clean(title: impl Into<String>, body: Markup) -> Markup {
    let style = std::fs::read_to_string("./src/server/static/style.css").unwrap();
    let title = title.into();
    html! {
        (DOCTYPE)
        html lang="da" {
            head {
                meta charset="utf-8";
                title { (title) }
                style { (style) }
            }
            body {
                (body)
            }
        }
    }
}

pub(crate) fn header(back: Option<&str>, user_name: &str) -> Markup {
    html! {
        div style="display:flex;justify-content:space-between" {
            div {
                @if let Some(href) = back {
                    a href={ (href) } { "← Tilbage" }
                }
            }
            a href="/settings" { (user_name) }
        }
        hr;
    }
}
