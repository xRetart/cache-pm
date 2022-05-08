use {
    library::error::{ParseArch, ParseMetadata, Read},
    std::{
        fmt::{self, Display, Formatter},
        io,
    },
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Finding(Read),
    ParseSpec(ParseArch),
    ParseMetadata(ParseMetadata),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Finding(e) => write!(f, "finding: {}", e),
            Self::ParseSpec(e) => write!(f, "parsing specification: {}", e),
            Self::ParseMetadata(e) => write!(f, "parsing metadata: {}", e),
        }
    }
}
