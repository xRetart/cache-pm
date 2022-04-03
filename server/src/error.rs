use {
    library::error::{Append, Extract, ParseArch, Read, Unpack, UnpackArchive, Write},
    std::{
        fmt::{self, Display, Formatter},
        io,
        num::ParseIntError,
    },
};

pub enum Error {
    Io(io::Error),
    Read(Read),
    Write(Write),
    Append(Append),
    Extract(Extract),
    Unpack(Unpack),
    UnpackArchive(UnpackArchive),
    ParseArch(ParseArch),
    ParseVersion(ParseIntError),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Read(e) => write!(f, "reading: {}", e),
            Self::Write(e) => write!(f, "writing: {}", e),
            Self::Append(e) => write!(f, "appending: {}", e),
            Self::Extract(e) => write!(f, "extracting: {}", e),
            Self::Unpack(e) => write!(f, "unpacking: {}", e),
            Self::UnpackArchive(e) => write!(f, "unpacking archive: {}", e),
            Self::ParseArch(e) => write!(f, "parsing architecture: {}", e),
            Self::ParseVersion(e) => write!(f, "parsing version: {}", e),
        }
    }
}
