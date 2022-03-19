use {
    std::{fs::{File, OpenOptions}, path::Path, io},
    crate::{error, Package, package::{Metadata, Build, Spec}},
};

pub struct Archive {
    file: File,
}
impl Archive {
    fn write(&mut self, package: &Package) -> Result<(), error::Write> {
        use {rkyv::to_bytes, std::io::Write};

        let serialized = to_bytes::<_, 8192>(package).map_err(error::Write::Serialize)?;
        self.file.write_all(&serialized).map_err(error::Write::Io)
    }

    pub fn open<P>(path: P, options: &OpenOptions) -> io::Result<Self>
    where P: AsRef<Path>
    {
        options.open(path).map(|file| Self { file })
    }
    pub fn create<P>(path: P, metadata: Metadata) -> Result<Self, error::Write>
    where
        P: AsRef<Path>,
    {
        let mut new = Self { file: File::create(path).map_err(error::Write::Io)? };
        new.write(&Package::empty(metadata)).map(|()| new)
    }
    pub fn read(&mut self) -> Result<Package, error::Read> {
        use {rkyv::from_bytes, std::io::Read};

        let mut buffer = Vec::new();
        self.file.read_to_end(&mut buffer).map_err(error::Read::Io)?;
        from_bytes(&buffer).map_err(error::Read::Deserialize)
    }
    pub fn append(&mut self, spec: Spec, build: Build) -> Result<(), error::Append> {
        use std::io::Seek;

        let mut package = self.read().map_err(error::Append::Read)?;

        package.distributions.insert(spec, build);

        self.file.rewind().map_err(error::Write::Io).map_err(error::Append::Write)?;
        self.write(&package).map_err(error::Append::Write)
    }
}
