use {
    confy::ConfyError,
    library::error::{Info, ParseArch, Query, UnpackArchive},
    std::io,
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),

    #[error("unpacking: {0}")]
    Unpack(#[from] UnpackArchive),

    #[error("parsing specification: {0}")]
    ParseSpec(#[from] ParseArch),

    #[error("sqlite3: {0}")]
    SQLite3(#[from] sqlite3::Error),

    #[error("info: {0}")]
    Info(#[from] Info),

    #[error("querying: {0}")]
    Query(#[from] Query),

    #[error("confy: {0}")]
    Confy(#[from] ConfyError),

    #[error("installation script failed")]
    InstallScript,

    #[error("package not found")]
    PkgNotFound,
}
