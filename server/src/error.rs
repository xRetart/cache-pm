use {
    library::error::{ParseArch, ParseMetadata, Read},
    std::io,
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),

    #[error("finding: {0}")]
    Finding(#[from] Read),

    #[error("parsing specification: {0}")]
    ParseSpec(#[from] ParseArch),

    #[error("parsing metadata: {0}")]
    ParseMetadata(#[from] ParseMetadata),
}
