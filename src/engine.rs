use axum_template::engine::Engine;
use minijinja_autoreload::AutoReloader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Context;

pub type AppEngine = Engine<AutoReloader>;
