use maud::{html, Markup};


pub fn search_bar() -> Markup {
    html!{
        div class="w-100% relative flex" {
            input class="p5px h20px w100% b-3px b-rd-2 b-s-solid outline-none" type="text" placeholder="Search package..";
            button class="w40px h36px text-center cursor-pointer" { "🔎" }
        }
    }
}

