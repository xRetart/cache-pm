use {
    crate::{
        error,
        package::{Build, Metadata, Package, Spec},
    },
    std::path::Path,
};

pub fn write<P>(path: P, package: &Package) -> Result<(), error::Write>
where
    P: AsRef<Path>,
{
    use {
        rkyv::to_bytes,
        std::{fs::File, io::Write},
    };

    let archive = to_bytes::<_, 1024>(package).map_err(error::Write::Serialize)?;
    File::create(path)
        .and_then(|mut file| file.write_all(&archive))
        .map_err(error::Write::Io)
}
pub fn create<P>(path: P, metadata: Metadata) -> Result<(), error::Write>
where
    P: AsRef<Path>,
{
    write(path, &Package::empty(metadata))
}
pub fn read<P>(path: P) -> Result<Package, error::Read>
where
    P: AsRef<Path>,
{
    use {
        rkyv::from_bytes,
        std::{fs::File, io::Read},
    };

    let mut buffer = Vec::new();
    File::open(path)
        .and_then(|mut file| file.read_to_end(&mut buffer))
        .map_err(error::Read::Io)?;

    from_bytes(&buffer).map_err(error::Read::Deserialize)
}
pub fn append<P>(path: P, spec: Spec, build: Build) -> Result<(), error::Append>
where
    P: AsRef<Path>,
{
    let mut package = read(&path).map_err(error::Append::Read)?;
    package.distributions.insert(spec, build);
    write(path, &package).map_err(error::Append::Write)
}
