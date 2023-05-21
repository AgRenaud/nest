use serde::Serialize;


#[derive(Serialize)]
struct User {
    name: String,
    password: String,
    email: String
}