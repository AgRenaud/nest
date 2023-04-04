use std::str::FromStr;

use crate::app_state::AppState;

use axum::body::Bytes;
use axum::{extract::State, response::Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::package::PkgFile;

#[derive(Serialize)]
pub struct SimpleIndex {
    packages: Vec<String>,
}


#[derive(TryFromMultipart)]
pub struct RequestData {
    #[form_data(field_name = ":action")]
    action: String,

    name: String,
    version: String,
    filetype: String,
    pyversion: String,
    metadata_version: String,
    summary: String,
    home_page: String,
    author: String,
    author_email: String,
    maintainer: String,
    maintainer_email: String,
    license: String,
    description: String,
    keywords: String,

    classifiers: Vec<String>,
    download_url: String,
    comment: String,
    sha256_digest: String,
    requires_python: String,
    description_content_type: String,
    md5_digest: String,
    blake2_256_digest: String,
    protocol_version: String,
    content: FieldData<Bytes>,
}

pub async fn upload(State(mut state): State<AppState>, data: TypedMultipart<RequestData>) {
    let content = data.0.content;
    let metadata = content.metadata;
    let filename = &metadata.file_name.unwrap();
    let bytes = content.contents;

    println!("Add package {}", data.0.name);
    println!(
        "file name = '{}', content type = '{}', size = '{}'",
        filename,
        &metadata.content_type.unwrap_or(String::from("text/plain")),
        &bytes.len()
    );


    state.save_file(filename, bytes).await;
    let created = state.db
        .create::<PkgFile>("packages")
        .content::<PkgFile>(PkgFile {
            pkgname: data.0.name.into(),
            version: data.0.version.into(),
            fullname: String::from_str("").expect("Error"),
            root: String::from_str("").unwrap(),
            relfn: String::from_str("").unwrap(),
            replaces: String::from_str("").unwrap(),
            pkgname_norm: String::from_str("").unwrap(),
            digest: String::from_str("").unwrap(),
            relfn_unix: String::from_str("").unwrap(),
            parsed_version: String::from_str("").unwrap(),
        }).await.unwrap();
}


pub async fn list_packages(State(state): State<AppState>) -> Json<SimpleIndex> {
    let packages: Vec<String> = Vec::new();

    Json(SimpleIndex { packages })
}
