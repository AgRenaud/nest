use serde::{Deserialize, Serialize};
use std::borrow::Cow;


#[derive(Serialize, Deserialize)]
pub struct PkgFile {
    pub pkgname: Cow<'static, str>,
    pub version: Cow<'static, str>,
    pub fullname: Cow<'static, str>,
    pub root: Cow<'static, str>,
    pub relfn: Cow<'static, str>,
    pub replaces: Cow<'static, str>,
    pub pkgname_norm: Cow<'static, str>,
    pub digest: Cow<'static, str>,
    pub relfn_unix: Cow<'static, str>,
    pub parsed_version: Cow<'static, str>,
}
