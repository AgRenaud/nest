use axum::{extract::Extension, Form};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;

use crate::components::header;
use sqlx::PgPool;

pub async fn sign_up() -> Markup {
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
                form hx-post="/manage/create_user" hx-swap="outerHTML" class="position-absolute shadow-2xl border-rd-1.2 p-10" {
                    div {
                        label for="userame" class="font-bold font-size-5" { "Username" }
                        input type="text" placeholder="Enter Username" name="username" required class="border-rd-1.2 m-4 p-2";
                    }
                    div {
                        label for="password" class="font-bold font-size-5" { "Password" }
                        input type="password" placeholder="Enter Password" name="password" required class="border-rd-1.2 m-4 p-2";
                    }
                    div {
                        label for="confirm_password" class="font-bold font-size-5" { "Confirm Password" }
                        input type="password" placeholder="Validate Password" name="confirm_password" required class="border-rd-1.2 m-4 p-2";
                    }
                    div {
                        button type="submit" class="float-left w32 h14 bg-transparent border-rd-1.2 border-2" { "Confirm" }
                        a href="/manage/sign_in" class="float-right" { "Already have an account ? Sign in" }
                    }
                }
            }
        }
    )
}

#[derive(Deserialize)]
pub struct SignUp {
    username: String,
    password: String,
    confirm_password: String,
}

#[tracing::instrument(name = "Manage::Create user", skip(pool, form))]
pub async fn create_user(Extension(pool): Extension<PgPool>, Form(form): Form<SignUp>) -> Markup {
    if form.password != form.confirm_password {
        todo!()
    }

    let user_created = sqlx::query!(
        r#"
            INSERT INTO users (username, password_hash)
            VALUES ($1::TEXT::CITEXT, $2)
        "#,
        &form.username,
        &form.password,
    )
    .execute(&pool)
    .await;

    match user_created {
        Ok(_) => html!(
            div class="position-absolute shadow-2xl border-rd-1.2 p-10" {
                p { "Welcome " (&form.username) }
                a href="/manage/sign_in" { "Click here to sign in !" }
            }
        ),
        Err(e) => html!(
            div class="position-absolute shadow-2xl border-rd-1.2 p-10" {
                p { "Unable to sign up. Please contact your administrator." } br;
                p { "Error message" i { (e.to_string()) } }
            }
        ),
    }
}
