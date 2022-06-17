use {
    library::error::{Info, ParseArch, UnpackArchive, Newest},
    std::io,
    confy::ConfyError,
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

    #[error("newest: {0}")]
    Newest(#[from] Newest),

    #[error("confy: {0}")]
    Confy(#[from] ConfyError),

    #[error("installation script failed")]
    InstallScript,

    #[error("package not found")]
    PkgNotFound,
}
