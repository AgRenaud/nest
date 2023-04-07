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
    let created: PkgFile = state
        .db
        .create("package")
        .content(PkgFile {
            pkgname: data.0.name.clone().into(),
            version: data.0.version.clone().into(),
            fullname: "".into(),
            root: "".into(),
            relfn: "".into(),
            replaces: "".into(),
            pkgname_norm: "".into(),
            digest: "".into(),
            relfn_unix: "".into(),
            parsed_version: "".into(),
        })
        .await
        .unwrap();

    let created: UploadData = state
        .db
        .create("package")
        .content(UploadData {
            protocol_version: data.0.protocol_version.into(),
            name: data.0.name.into(),
            version: data.0.version.into(),
            filetype: data.0.filetype.into(),
            pyversion: data.0.pyversion.into(),
            metadata_version: data.0.metadata_version.into(),
            summary: data.0.summary.into(),
            home_page: data.0.home_page.into(),
            author: data.0.author.into(),
            author_email: data.0.author_email.into(),
            maintainer: data.0.maintainer.into(),
            maintainer_email: data.0.maintainer_email.into(),
            license: data.0.license.into(),
            description: data.0.description.into(),
            keywords: data.0.keywords.into(),
            platform: match data.0.platform {
                Some(x) => x.into(),
                _ => "".into(),
            },
            download_url: data.0.download_url.into(),
            supported_platform: match data.0.supported_platform {
                Some(x) => x.into(),
                _ => "".into(),
            },
            comment: data.0.comment.into(),
            md5_digest: data.0.md5_digest.into(),
            sha256_digest: data.0.sha256_digest.into(),
            blake2_256_digest: data.0.blake2_256_digest.into(),
            provides: data.0.provides.into(),
            requires: data.0.requires.into(),
            obsoletes: data.0.obsoletes.into(),
            project_urls: data.0.project_urls.into(),
            provides_dist: data.0.provides_dist.into(),
            obsoletes_dist: data.0.obsoletes_dist.into(),
            requires_dist: data.0.requires_dist.into(),
            requires_external: data.0.requires_external.into(),
            requires_python: data.0.requires_python.into(),
        })
        .await
        .unwrap();
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
