use maud::{html, Markup, DOCTYPE};

use crate::components::header;
use crate::search::search_bar;

pub async fn home() -> Markup {
    html!(
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@1.9.3" {};
            script src="https://cdn.jsdelivr.net/npm/@unocss/runtime" {};
            title { "Nest" }
        }
        body class="m0 p0 font-sans" {
            (header())
            div class="w-30% absolute top-50% left-50% translate-x--50% translate-y--50%" {
                (search_bar())
            }
        }
    )
}
