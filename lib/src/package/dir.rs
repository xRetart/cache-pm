use {
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{io, path::Path},
};

/// A `Build` stores all data of a software
#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Dir {
    /// Path of the root of the directory
    pub path: String,

    /// The buffer for all data
    data: Vec<u8>,
}
impl Dir {
    /// Archives the directory at `path` and compresses it with a of compression level `compression`.
    /// # Errors
    /// Returns `std::io::Error` when opening the encoder `zstd::Encoder`,
    /// inserting the directory at `path` into the `tar::Builder` or
    /// finishing the `tar::Builder` failed.
    pub fn encode(path: String, compression: i32) -> io::Result<Self> {
        use {tar::Builder, zstd::Encoder};

        let buffer = Vec::new();

        // create archive builder with compression
        let mut builder = Builder::new(Encoder::new(buffer, compression)?);

        // add path to archive
        let rpath = &path;
        builder.append_dir_all(rpath, rpath)?;

        // finish streams and unpack data
        builder
            .into_inner()
            .and_then(Encoder::finish)
            .map(|data| Self { path, data })
    }
    /// Decodes a `Build` into a destination directory at path `dest`.
    /// # Errors
    /// Returns `std::io::Error` when opening the decoder `zstd::Decoder`
    /// or unpacking the archive fails.
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
