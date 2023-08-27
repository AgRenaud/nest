use axum::{extract::Extension, Form};
use maud::{html, Markup, DOCTYPE};
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::authentication;
use crate::components::header;

use sqlx::PgPool;

fn sign_up_form(error_message: Option<&str>) -> Markup {
    html! {
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

            @if let Some(error_message) = error_message {
                p class="color-red" { (error_message) }
            }

            div {
                button type="submit" class="float-left w32 h14 bg-transparent border-rd-1.2 border-2" { "Confirm" }
                a href="/manage/sign_in" class="float-right" { "Already have an account ? Sign in" }
            }
        }
    }
}

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
            div class="ma w-100" {
                (sign_up_form(None))
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
        let error_message = "Password are not the same. Please check your password.";
        return sign_up_form(Some(error_message));
    }

    let password = secrecy::Secret::new(form.password);
    let password_hash =
        authentication::compute_password_hash(password).expect("Unable to create a proper hash.");

    let user_created = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1::TEXT::CITEXT, $2)
        "#,
        &form.username,
        password_hash.expose_secret(),
    )
    .execute(&pool)
    .await;

    match user_created {
        Ok(_) => html! {
            div class="ma w-100 position-absolute shadow-2xl border-rd-1.2 p-10" {
                p { "Welcome " (&form.username) }
                a href="/manage/sign_in" { "Click here to sign in !" }
            }
        },
        Err(e) => {
            let err = e.into_database_error();

            let error_kind: &sqlx::error::ErrorKind = &err
                .as_ref()
                .map_or(sqlx::error::ErrorKind::Other, |err| err.kind());

            match &error_kind {
                sqlx::error::ErrorKind::UniqueViolation => {
                    let error_message = format!("User {} already exists.", &form.username);
                    sign_up_form(Some(&error_message))
                }
                _ => html! {
                    div class="ma w-100 position-absolute shadow-2xl border-rd-1.2 p-10" {
                        p { "Unable to sign up. Please contact your administrator." } br;
                        p { "Error message" br; i { @if let Some(err) = err { (err.message())} } }
                    }
                },
            }
        }
    }
}
