use {
    library::error::{Append, Extract, ParseArch, Read, Unpack, UnpackArchive, Write},
    std::{io, num::ParseIntError},
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),

    #[error("reading: {0}")]
    Read(#[from] Read),

    #[error("writing: {0}")]
    Write(#[from] Write),

    #[error("appending: {0}")]
    Append(#[from] Append),

    #[error("extracting: {0}")]
    Extract(#[from] Extract),

    #[error("unpacking: {0}")]
    Unpack(#[from] Unpack),

    #[error("unpacking archive: {0}")]
    UnpackArchive(#[from] UnpackArchive),

    #[error("parssing architecture: {0}")]
    ParseArch(#[from] ParseArch),

    #[error("parsing version: {0}")]
    ParseVersion(#[from] ParseIntError),
}
