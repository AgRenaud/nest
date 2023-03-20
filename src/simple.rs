use crate::app_state::AppState;


use axum::{extract::State, response::Json};
use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};
use serde::Serialize;


#[derive(Serialize)]
pub struct SimpleIndex {
    packages: Vec<String>
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
    content: FieldData<TempFile>,
}

pub async fn upload(data: TypedMultipart<RequestData>) {
    println!("- New request -");
    println!("\t{:?}", data.0.action);
    println!("\t{:?}", data.0.name);
    println!("\t{:?}", data.0.version);
    println!("\t{:?}", data.0.filetype);
    println!("\t{:?}", data.0.pyversion);
    println!("\t{:?}", data.0.metadata_version);
    println!("\t{:?}", data.0.summary);
    println!("\t{:?}", data.0.home_page);
    println!("\t{:?}", data.0.author);
    println!("\t{:?}", data.0.author_email);
    println!("\t{:?}", data.0.maintainer);
    println!("\t{:?}", data.0.maintainer_email);
    println!("\t{:?}", data.0.license);
    println!("\t{:?}", data.0.description);
    println!("\t{:?}", data.0.keywords);
    println!("\t{:?}", data.0.classifiers);
    println!("\t{:?}", data.0.download_url);
    println!("\t{:?}", data.0.comment);
    println!("\t{:?}", data.0.sha256_digest);
    println!("\t{:?}", data.0.requires_python);
    println!("\t{:?}", data.0.description_content_type);
    println!("\t{:?}", data.0.md5_digest);
    println!("\t{:?}", data.0.blake2_256_digest);
    println!("\t{:?}", data.0.protocol_version);
    println!("\t{:?}", data.0.content.metadata.file_name);
}

pub async fn list_packages(State(state): State<AppState>) -> Json<SimpleIndex> {
    let path = state.index_dir;

    let packages: Vec<String> = Vec::new();

    Json(SimpleIndex{ packages })
}