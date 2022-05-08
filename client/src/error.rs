use {
    library::error::{ParseArch, UnpackArchive},
    std::{
        fmt::{self, Display, Formatter},
        io,
    },
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Unpack(UnpackArchive),
    ParseSpec(ParseArch),
    InstallScript,
    PkgNotFound,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Unpack(e) => write!(f, "unpacking: {}", e),
            Self::ParseSpec(e) => write!(f, "parsing specification: {}", e),
            Self::InstallScript => write!(f, "installation script failed"),
            Self::PkgNotFound => write!(f, "package not found"),
        }
    }
}
