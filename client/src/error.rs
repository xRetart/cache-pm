use {
    library::error::{Info, ParseArch, UnpackArchive, Newest},
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
    SQLite3(sqlite3::Error),
    Info(Info),
    Newest(Newest),
    InstallScript,
    PkgNotFound,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Unpack(e) => write!(f, "unpacking: {}", e),
            Self::ParseSpec(e) => write!(f, "parsing specification: {}", e),
            Self::SQLite3(e) => write!(f, "sqlite3: {}", e),
            Self::Info(e) => write!(f, "info: {}", e),
            Self::Newest(e) => write!(f, "Newest: {}", e),
            Self::InstallScript => write!(f, "installation script failed"),
            Self::PkgNotFound => write!(f, "package not found"),
        }
    }
}
