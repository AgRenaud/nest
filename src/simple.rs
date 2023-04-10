use std::str::FromStr;

use crate::app_state::AppState;

use crate::package::PkgFile;
use axum::body::Bytes;
use axum::extract::RawPathParams;
use axum::{extract::State, response::Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;

#[derive(Serialize)]
pub struct SimpleIndex {
    packages: Vec<String>,
}

#[derive(TryFromMultipart)]
pub struct RequestData {
    #[form_data(field_name = ":action")]
    action: String,
    protocol_version: String,

    // identify release
    name: String,
    version: String,

    // file content
    filetype: String,
    pyversion: String,

    // additional meta-data
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
    platform: Option<String>,
    //classifiers: String,
    download_url: String,
    supported_platform: Option<String>,
    comment: String,
    md5_digest: String,
    sha256_digest: String,
    blake2_256_digest: String,

    // PEP 314
    provides: String,
    requires: String,
    obsoletes: String,

    // Metadata 1.2
    project_urls: String,
    provides_dist: String,
    obsoletes_dist: String,
    requires_dist: String,
    requires_external: String,
    requires_python: String,
    content: FieldData<Bytes>,
}

#[derive(Serialize, Deserialize)]
struct UploadData {
    //action: Cow<'static, str>,
    protocol_version: Cow<'static, str>,
    name: Cow<'static, str>,
    version: Cow<'static, str>,
    filetype: Cow<'static, str>,
    pyversion: Cow<'static, str>,
    metadata_version: Cow<'static, str>,
    summary: Cow<'static, str>,
    home_page: Cow<'static, str>,
    author: Cow<'static, str>,
    author_email: Cow<'static, str>,
    maintainer: Cow<'static, str>,
    maintainer_email: Cow<'static, str>,
    license: Cow<'static, str>,
    description: Cow<'static, str>,
    keywords: Cow<'static, str>,
    platform: Cow<'static, str>,
    //classifiers: Cow<'static, str>,
    download_url: Cow<'static, str>,
    supported_platform: Cow<'static, str>,
    comment: Cow<'static, str>,
    md5_digest: Cow<'static, str>,
    sha256_digest: Cow<'static, str>,
    blake2_256_digest: Cow<'static, str>,
    provides: Cow<'static, str>,
    requires: Cow<'static, str>,
    obsoletes: Cow<'static, str>,
    project_urls: Cow<'static, str>,
    provides_dist: Cow<'static, str>,
    obsoletes_dist: Cow<'static, str>,
    requires_dist: Cow<'static, str>,
    requires_external: Cow<'static, str>,
    requires_python: Cow<'static, str>,
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
    state.db.query("SELECT * FROM maintainers").bind(bindings)
}

pub async fn list_packages(State(state): State<AppState>) -> Json<SimpleIndex> {
    let packages: Vec<String> = Vec::new();

    Json(SimpleIndex { packages })
}

pub async fn index(params: RawPathParams) -> axum::response::Html<&'static str> {
    println!("Looking at the index");
    dbg!(params);

    axum::response::Html("Hello World")
}
