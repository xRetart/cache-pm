use {
    crate::{
        error,
        package::{Dir, Metadata, Spec},
        Package,
    },
    std::{
        fs::{File, OpenOptions},
        io,
        path::Path,
    },
};

/// An `Archive` represents a package contained in a file
#[derive(Debug)]
pub struct Archive {
    file: File,
}
impl Archive {
    // writes package to `Archive::file`
    fn write(&mut self, package: &Package) -> Result<(), error::Write> {
        use {rkyv::to_bytes, std::io::Write};

        let serialized = to_bytes::<_, 8192>(package)?;
        self.file.write_all(&serialized)?;

        Ok(())
    }

    /// Open the `Archive` at a `path` with `options`
    /// # Errors
    /// Returns `std::io::Error` if the file at `path` does not exists or
    /// the user does not have permission to open it with `options`.
    pub fn open<P: AsRef<Path>>(path: P, options: &OpenOptions) -> io::Result<Self> {
        options.open(path).map(|file| Self { file })
    }

    /// Creates an empty `Archive` at `path` with specified `metadata`
    /// # Errors
    /// Returns `lib::error::Write::Io` if the file could not be created or
    /// could not be written to.
    /// Returns `lib::error::Write::Serialize` if converting the package to binary failed.
    pub fn create<P: AsRef<Path>>(path: P, metadata: Metadata) -> Result<Self, error::Write> {
        let mut new = Self {
            file: File::create(path)?,
        };
        new.write(&Package::empty(metadata)?).map(|()| new)
    }

    /// Reads the `Package` contained in the `Archive`
    /// # Errors
    /// Returns `lib::error::Read::Io` if reading the buffer of the file failed.
    /// Returns `lib::error::Read::Deserialize` if converting from binary to the Package failed.
    pub fn read(&mut self) -> Result<Package, error::Read> {
        use {rkyv::from_bytes, std::io::Read};

        let mut buffer = Vec::new();
        self.file.read_to_end(&mut buffer)?;
        from_bytes(&buffer).map_err(|e| e.into())
    }

    /// Unpacks the `Package` contained in the `Archive`
    /// # Errors
    /// Returns `lib::error::UnpackArchive::Read` if reading the `Package` from the file failed.
    /// Returns `lib::error::UnpackArchive::Package` if unpacking the `Package` failed.
    pub fn unpack<P: AsRef<Path>>(
        &mut self,
        dest: P,
        spec: &Spec,
    ) -> Result<(), error::UnpackArchive> {
        self.read()?.unpack(spec, dest.as_ref())?;
        Ok(())
    }

    /// Appends a `Dist` to the package in the `Archive`
    /// # Errors
    /// Returns `lib::error::Append::Read` if reading the `Archive` failed.
    /// Returns `lib::error::Append::Write` if writing the updated package to the `Archive` failed.
    pub fn append(&mut self, spec: Spec, build: Dir) -> Result<(), error::Append> {
        use std::io::Seek;

        let mut package = self.read()?;

        package.distributions.insert(spec, build);

        self.file.rewind().map_err(error::Write::Io)?;
        self.write(&package)?;
        Ok(())
    }
}
