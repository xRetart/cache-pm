use std::{io, fmt::{self, Formatter, Display}};


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
        }
    }
}
