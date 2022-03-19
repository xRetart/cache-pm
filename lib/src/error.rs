use {
    crate::package::Package,
    rkyv::{
        ser::serializers::{AllocScratchError, CompositeSerializerError, SharedSerializeMapError},
        validation::validators::FromBytesError,
    },
    std::{convert::Infallible, io},
};

/// An Error that can occur while reading an archive.
#[derive(Debug)]
pub enum Read {
    Deserialize(FromBytesError<'static, Package>),
    Io(io::Error),
}
/// An Error that can occur while writing an archive.
#[derive(Debug)]
pub enum Write {
    Serialize(CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>),
    Io(io::Error),
}
/// An Error that can occur while appending to an archive.
#[derive(Debug)]
pub enum Append {
    Read(Read),
    Write(Write),
}

/// An Error that can occur while unpacking an archive.
#[derive(Debug)]
pub enum Unpacking {
    SpecNotFound,
    Io(io::Error),
}
