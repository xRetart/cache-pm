use crate::Error;

pub fn read(path: &str) -> Result<(), Error> {
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(path, OpenOptions::new().read(true))
        .map_err(Error::Io)
        .and_then(|mut archive| archive.read().map_err(Error::Read))
        .map(|package| println!("{}", package))
}

pub fn create(path: &str, name: String, vers: &str) -> Result<(), Error> {
    use library::{package::Metadata, Archive};

    let version = vers.parse().map_err(Error::ParseVersion)?;
    Archive::create(path, Metadata { name, version })
        .map_err(Error::Write)
        .map(|_| ())
}
pub fn append(path: &str, build: String, spec: &str) -> Result<(), Error> {
    use {
        library::{package::Dir, Archive},
        std::fs::OpenOptions,
    };

    Archive::open(path, OpenOptions::new().read(true).write(true))
        .map_err(Error::Io)?
        .append(
            spec.parse().map_err(Error::ParseArch)?,
            Dir::encode(build, 9).map_err(Error::Io)?,
        )
        .map_err(Error::Append)
}
pub fn unpack(source: &str, dest: &str, spec: &str) -> Result<(), Error> {
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(source, OpenOptions::new().read(true))
        .map_err(Error::Io)?
        .unpack(dest, &spec.parse().map_err(Error::ParseArch)?)
        .map_err(Error::UnpackArchive)
}
