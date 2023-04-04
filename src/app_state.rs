use bytes::Bytes;
use object_store::{path::Path, ObjectStore};
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Clone)]
pub struct AppState {
    pub static_dir: String,
    pub store: Arc<dyn ObjectStore>,
    pub db: Arc<Surreal<Client>>,
}

impl AppState {
    pub async fn save_file(&mut self, filename: &String, bytes: Bytes) {
        let path = Path::from_url_path(filename).unwrap();

        self.store.put(&path, bytes).await.unwrap();
    }
}
