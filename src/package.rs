use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct PkgFile {
    pub pkgname: String,
    pub version: String,
    pub fullname: String,
    pub root: String,
    pub relfn: String,
    pub replaces: String,
    pub pkgname_norm: String,
    pub digest: String,
    pub relfn_unix: String,
    pub parsed_version: String
}
