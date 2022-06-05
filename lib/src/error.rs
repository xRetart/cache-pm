use {
    crate::package::{metadata::Version, Package},
    rkyv::{
        ser::serializers::{AllocScratchError, CompositeSerializerError, SharedSerializeMapError},
        validation::validators::FromBytesError,
    },
    std::{convert::Infallible, io, str::FromStr},
    thiserror::Error,
};

/// An Error that can occur while reading an archive.
#[derive(Debug, Error)]
pub enum Read {
    #[error("deserializing: {0}")]
    Deserialize(#[from] FromBytesError<'static, Package>),

    #[error("io: {0}")]
    Io(#[from] io::Error),
}

/// An Error that can occur while writing an archive.
#[derive(Debug, Error)]
pub enum Write {
    #[error("serializing: {0}")]
    Serialize(
        #[from] CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>,
    ),

    #[error("io: {0}")]
    Io(#[from] io::Error),
}

/// An Error that can occur while appending to an archive.
#[derive(Debug, Error)]
pub enum Append {
    #[error("reading: {0}")]
    Read(#[from] Read),

    #[error("writing: {0}")]
    Write(#[from] Write),
}
/// An Error that can occur while reading an archive.
#[derive(Debug, Error)]
pub enum Extract {
    #[error("reading: {0}")]
    Read(#[from] Read),

    #[error("io: {0}")]
    Io(#[from] io::Error),
}

/// An Error that can occur while unpacking an archive.
#[derive(Debug, Error)]
pub enum Unpack {
    #[error("Specification is not available.")]
    SpecNotFound,

    #[error("io: {0}")]
    Io(#[from] io::Error),
}
/// An Error that can occur while unpacking an archive.
#[derive(Debug, Error)]
pub enum UnpackArchive {
    #[error("reading: {0}")]
    Read(#[from] Read),

    #[error("package: {0}")]
    Package(#[from] Unpack),
}

/// An error that can occur while parsing an architecture.
#[derive(Debug, Error)]
pub enum ParseArch {
    #[error("Architecture is unknown.")]
    Unknown,
}

/// An error that can occur while parsing metadata
#[derive(Debug, Error)]
pub enum ParseMetadata {
    #[error("version: {0}")]
    Version(#[from] <Version as FromStr>::Err),

    #[error("Version format is incorrect.")]
    Format,
}

/// An error that can occur while retrieving information from database
#[derive(Debug, Error)]
pub enum Info {
    #[error("sqlite3: {0}")]
    SQLite3(#[from] sqlite3::Error),

    #[error("Package could not be found.")]
    PackageNotFound,

    #[error("Column is invalid.")]
    InvalidColumn,
}

/// An error that can occur while retrieving the newest version for a package from the database
#[derive(Debug, Error)]
pub enum Newest {
    #[error("sqlite3: {0}")]
    SQLite3(#[from] sqlite3::Error),

    #[error("Version format in database is invalid.")]
    Version(#[from] <Version as FromStr>::Err),

    #[error("Package could not be found.")]
    PackageNotFound,

    #[error("Column is invalid.")]
    InvalidColumn,
}
