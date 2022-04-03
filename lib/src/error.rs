use {
    crate::package::Package,
    rkyv::{
        ser::serializers::{AllocScratchError, CompositeSerializerError, SharedSerializeMapError},
        validation::validators::FromBytesError,
    },
    std::{
        convert::Infallible,
        fmt::{self, Display, Formatter},
        io,
    },
};

/// An Error that can occur while reading an archive.
#[derive(Debug)]
pub enum Read {
    Deserialize(FromBytesError<'static, Package>),
    Io(io::Error),
}
impl Display for Read {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Deserialize(e) => write!(f, "deserializing: {}", e),
            Self::Io(e) => write!(f, "io: {}", e),
        }
    }
}

/// An Error that can occur while writing an archive.
#[derive(Debug)]
pub enum Write {
    Serialize(CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>),
    Io(io::Error),
}
impl Display for Write {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Serialize(e) => write!(f, "serializing: {}", e),
            Self::Io(e) => write!(f, "io: {}", e),
        }
    }
}

/// An Error that can occur while appending to an archive.
#[derive(Debug)]
pub enum Append {
    Read(Read),
    Write(Write),
}
impl Display for Append {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Read(e) => write!(f, "reading: {}", e),
            Self::Write(e) => write!(f, "writing: {}", e),
        }
    }
}
/// An Error that can occur while reading an archive.
#[derive(Debug)]
pub enum Extract {
    Read(Read),
    Io(io::Error),
}
impl Display for Extract {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Read(e) => write!(f, "reading: {}", e),
            Self::Io(e) => write!(f, "io: {}", e),
        }
    }
}

/// An Error that can occur while unpacking an archive.
#[derive(Debug)]
pub enum Unpack {
    SpecNotFound,
    Io(io::Error),
}
impl Display for Unpack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SpecNotFound => f.write_str("specification is not available."),
            Self::Io(e) => write!(f, "io: {}", e),
        }
    }
}
/// An Error that can occur while unpacking an archive.
#[derive(Debug)]
pub enum UnpackArchive {
    Read(Read),
    Package(Unpack),
}
impl Display for UnpackArchive {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Read(e) => write!(f, "read: {}", e),
            Self::Package(e) => write!(f, "package: {}", e),
        }
    }
}

/// An error that can occur while parsing an architecture.
#[derive(Debug)]
pub enum ParseArch {
    Unknown,
}
impl Display for ParseArch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Unknown architecture.")
    }
}
