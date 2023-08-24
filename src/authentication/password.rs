use std::error::Error;
use std::fmt;

use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::telemetry::spawn_blocking_with_tracing;


#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UnexpectedError,
    UnknownUsername
}


impl Error for AuthError {}


impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid credentials."),
            AuthError::UnexpectedError => write!(f, "An unexpected error occurred."),
            AuthError::UnknownUsername => write!(f, "Unknown username.")
        }
    }
}


pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, AuthError> {
    let row = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| AuthError::UnexpectedError)?
    .map(|row| (row.id, Secret::new(row.password_hash)));
    Ok(row)
}


#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    let verify_result = spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await;

    match verify_result {
        Ok(Ok(())) => user_id.ok_or(AuthError::UnknownUsername),
        Ok(Err(_)) => Err(AuthError::InvalidCredentials),
        Err(_) => Err(AuthError::UnexpectedError),
    }
}


#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .map_err(|_| AuthError::InvalidCredentials)?;

    let argon2 = Argon2::default();
    let password_bytes = password_candidate.expose_secret().as_bytes();

    if argon2.verify_password(password_bytes, &expected_password_hash).is_ok() {
        Ok(())
    } else {
        Err(AuthError::InvalidCredentials)
    }
}


#[tracing::instrument(name = "Change password", skip(password, pool))]
pub async fn change_password(
    user_id: uuid::Uuid,
    password: Secret<String>,
    pool: &PgPool,
) -> Result<(), AuthError> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await
        .map_err(|_| AuthError::UnexpectedError)??;

    let result = sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE id = $2
        "#,
        password_hash.expose_secret(),
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| AuthError::UnexpectedError)?;

    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(AuthError::UnknownUsername)
    }
}

fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, AuthError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    );

    let password_hash = argon
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .expect("Unable to hash password")
        .to_string();

    Ok(Secret::new(password_hash))
}
