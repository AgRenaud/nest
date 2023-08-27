use maud::{html, Markup};

pub fn header() -> Markup {
    html! (
        header class="font-sans font-bold bg-black flex justify-between items-center pl-2rem pr-2rem" {
            a href="/" class="font-extrabold font-size-8 color-white hover:cursor-pointer decoration-none" { "Nest 🪺" }
            ul class="list-none"{
                li class="inline-block p20px" { a href="/manage/sign_up" class="font-size-5 color-white hover:color-emerald decoration-none" { "Sign up" } }
                li class="inline-block p20px" { a href="/manage/sign_in" class="font-size-5 color-white hover:color-emerald decoration-none" { "Sign in" } }
            }
        }
    )
}
