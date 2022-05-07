use {
    crate::error::Error,
    rkyv::{
        Serialize,
        AlignedVec,
        ser::serializers::{
            CompositeSerializer,
            FallbackScratch,
            AlignedSerializer,
            SharedSerializeMap,
            HeapScratch,
            AllocScratch,
        },
    },
    std::{net::TcpStream, io::BufReader, str::FromStr},
};


#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}
impl Connection {
    pub fn open(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub fn reader(&self) -> BufReader<&TcpStream> {
        BufReader::new(&self.stream)
    }
    pub fn receive<I, E>(reader: &mut BufReader<&TcpStream>, conv_err: E) -> Result<I, Error>
    where
        I: FromStr,
        E: FnOnce(<I as FromStr>::Err) -> Error,
    {
        use std::io::BufRead;

        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .map_err(Error::Io)
            .and_then(|_| buf.trim().parse().map_err(conv_err))
    }
    pub fn send<I>(&mut self, item: I) -> Result<(), Error>
    where
        I: Serialize<
            CompositeSerializer<
                AlignedSerializer<AlignedVec>,
                FallbackScratch<HeapScratch<8192_usize>, AllocScratch>,
                SharedSerializeMap
            >
        > + std::fmt::Debug
    {
        use {std::io::Write, rkyv::to_bytes};

        const SCRATCH_SPACE: usize = 2 << 12;

        let serialized = to_bytes::<_, SCRATCH_SPACE>(&item).map_err(Error::Serializing)?;
        self.stream
            .write_all(&serialized)
            .map_err(Error::Io)
            .map(|_| ())
    }
}
