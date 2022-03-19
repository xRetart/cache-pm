use {
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{io, path::Path},
};

#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Build {
    data: Vec<u8>,
    compression: i32,
}
impl Build {
    pub fn archive<P>(path: P, compression: i32) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        use {tar::Builder, zstd::Encoder};

        let buffer = Vec::new();

        // create archive builder with compression
        let mut builder = Builder::new(Encoder::new(buffer, compression)?);

        // add path to archive
        let path = &path;
        builder.append_dir(path, path)?;

        // finish streams and unpack data
        builder
            .into_inner()
            .and_then(Encoder::finish)
            .map(|data| Self { data, compression })
    }
    pub fn decode<P>(&self, dest: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        use {tar::Archive, zstd::Decoder};

        let mut archive = Archive::new(Decoder::new(self.data.as_slice())?);
        archive.unpack(dest)?;

        archive.into_inner().finish();
        Ok(())
    }
}
