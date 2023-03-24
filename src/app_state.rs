use std::sync::Arc;
use bytes::Bytes;
use object_store::{ObjectStore, path::Path};



#[derive(Clone)]
pub struct AppState {
    pub static_dir: String,
    pub store: Arc<dyn ObjectStore>
}

impl AppState {
    pub async fn save_file(&mut self, filename: &String, bytes: Bytes) {

        let path = Path::from_url_path(filename).unwrap();

        self.store
            .put(&path, bytes)
            .await
            .unwrap();
    }

}