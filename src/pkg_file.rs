

struct PkgFile {
    pkgname: String,
    version: String,
    fullname: String,
    root: String,
    relfn: String,
    replaces: String,
    pkgname_norm: String,
    digest: String,
    relfn_unix: String,
    parsed_version: String,
    digester: String,
    parsed_version: String,
    relfn_unix: String,
}

    // def __init__(
    //     self,
    //     pkgname: str,
    //     version: str,
    //     fn: t.Optional[str] = None,
    //     root: t.Optional[str] = None,
    //     relfn: t.Optional[str] = None,
    //     replaces: t.Optional["PkgFile"] = None,
    // ):
    //     self.pkgname = pkgname
    //     self.pkgname_norm = normalize_pkgname(pkgname)
    //     self.version = version
    //     self.parsed_version: tuple = parse_version(version)
    //     self.fn = fn
    //     self.root = root
    //     self.relfn = relfn
    //     self.relfn_unix = None if relfn is None else relfn.replace("\\", "/")
    //     self.replaces = replaces
    //     self.digest = None
    //     self.digester = None
    // 
    // def __repr__(self) -> str:
    //     return "{}({})".format(
    //         self.__class__.__name__,
    //         ", ".join(
    //             [
    //                 f"{k}={getattr(self, k, 'AttributeError')!r}"
    //                 for k in sorted(self.__slots__)
    //             ]
    //         ),
    //     )
    // 
    // @property
    // def fname_and_hash(self) -> str:
    //     if self.digest is None and self.digester is not None:
    //         self.digest = self.digester(self)
    //     hashpart = f"#{self.digest}" if self.digest else ""
    //     return self.relfn_unix + hashpart  # type: ignor""e"