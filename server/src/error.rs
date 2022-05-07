use {
    library::error::{Read, ParseMetadata, ParseArch},
    rkyv::ser::serializers::{AllocScratchError, CompositeSerializerError, SharedSerializeMapError},
    std::{
        io,
        fmt::{self, Formatter, Display},
        convert::Infallible,
    }
};


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Finding(Read),
    ParseSpec(ParseArch),
    ParseMetadata(ParseMetadata),
    Serializing(CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Finding(e) => write!(f, "finding: {}", e),
            Self::ParseSpec(e) => write!(f, "parsing specification: {}", e),
            Self::ParseMetadata(e) => write!(f, "parsing metadata: {}", e),
            Self::Serializing(e) => write!(f, "serializing: {}", e),
        }
    }
}
