use {crate::Error, std::path::Path};

pub fn read<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(path, OpenOptions::new().read(true))
        .map_err(Error::Io)
        .and_then(|mut archive| archive.read().map_err(Error::Read))
        .map(|package| println!("{}", package))
}

pub fn create<P, V>(path: P, name: String, vers: V) -> Result<(), Error>
where
    P: AsRef<Path>,
    V: AsRef<str>,
{
    use library::{package::Metadata, Archive};

    let version = vers.as_ref().parse().map_err(Error::ParseVersion)?;
    Archive::create(path, Metadata { name, version })
        .map_err(Error::Write)
        .map(|_| ())
}
pub fn append<P, S, B>(path: P, build: B, spec: S) -> Result<(), Error>
where
    P: AsRef<Path>,
    S: AsRef<str>,
    B: AsRef<str>,
{
    use {
        library::{package::Build, Archive},
        std::fs::OpenOptions,
    };

    Archive::open(path, OpenOptions::new().read(true).write(true))
        .map_err(Error::Io)?
        .append(
            spec.as_ref().parse().map_err(Error::ParseArch)?,
            Build::encode(build.as_ref(), 9).map_err(Error::Io)?,
        )
        .map_err(Error::Append)
}
pub fn unpack<P, S, D>(path: P, dest: D, spec: S) -> Result<(), Error>
where
    P: AsRef<Path>,
    S: AsRef<str>,
    D: AsRef<str>,
{
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(path, OpenOptions::new().read(true))
        .map_err(Error::Io)?
        .read()
        .map_err(Error::Read)?
        .unpack(
            &spec.as_ref().parse().map_err(Error::ParseArch)?,
            dest.as_ref(),
        )
        .map_err(Error::Unpack)
}
