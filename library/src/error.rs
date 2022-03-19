use {
    crate::package::Package,
    rkyv::{
        ser::serializers::{AllocScratchError, CompositeSerializerError, SharedSerializeMapError},
        validation::validators::FromBytesError,
    },
    std::{convert::Infallible, io},
};

#[derive(Debug)]
pub enum Read {
    Deserialize(FromBytesError<'static, Package>),
    Io(io::Error),
}
#[derive(Debug)]
pub enum Write {
    Serialize(CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>),
    Io(io::Error),
}
#[derive(Debug)]
pub enum Append {
    Read(Read),
    Write(Write),
}
#[derive(Debug)]
pub enum Unpacking {
    SpecNotFound,
    Io(io::Error),
}
