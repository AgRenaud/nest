use maud::{html, Markup, DOCTYPE};

pub async fn home() -> Markup {
    html!(
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@1.9.3" {};
            script src="https://cdn.jsdelivr.net/npm/@unocss/runtime" {};
            title { "Nest" }
        }
        body {
            a href="/manage/sign_in" { "sign in" }
            br;
            a href="/manage/sign_up" { "sign up" }
        }
    )
}

