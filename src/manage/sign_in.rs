#![allow(dead_code)]
#![allow(unused_variables)]

use axum::{Extension, Form};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use crate::components::header;

pub async fn sign_in() -> Markup {
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
            h1 class="w-full font-extrabold font-size-8 color-black" { "Sign In"}
            div class="ma w-100 " {
                form hx-post="/manage/login" class="position-absolute shadow-2xl border-rd-1.2 p-10" {
                    div {
                        label for="uname" class="font-bold font-size-5" { "Username" }
                        input type="text" placeholder="Enter Username" name="uname" required class="border-rd-1.2 m-4 p-2";
                    }
                    div {
                        label for="psw" class="font-bold font-size-5" { "Password" }
                        input type="password" placeholder="Enter Password" name="psw" required class="border-rd-1.2 m-4 p-2";
                    }
                    div {
                        button type="submit" class="float-left w32 h14 bg-transparent border-rd-1.2 border-2" { "Login" }
                        a href="#" class="float-right" { "Forgot Password?" }
                        br;
                        a href="/manage/sign_up" class="float-right" { "Create an account" }
                    }
                }
            }
        }
    )
}

#[derive(Deserialize)]
pub struct SignIn {
    username: String,
    password: String,
}

#[tracing::instrument(name = "Manage::User Connect", skip(pool, form), fields(username=%form.username))]
pub async fn connect(Extension(pool): Extension<PgPool>, Form(form): Form<SignIn>) -> Markup {
    todo!()
}
