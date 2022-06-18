use crate::Error;

pub fn read(path: &str) -> Result<(), Error> {
    use {library::Archive, std::fs::OpenOptions};

    println!(
        "{}",
        Archive::open(path, OpenOptions::new().read(true))?.read()?
    );

    Ok(())
}

pub fn create(path: &str, name: String, vers: &str) -> Result<(), Error> {
    use library::{package::Metadata, Archive};

    let version = vers.parse()?;
    Archive::create(path, Metadata { name, version })?;

    Ok(())
}
pub fn append(path: &str, build: String, spec: &str) -> Result<(), Error> {
    use {
        library::{package::Dir, Archive},
        std::fs::OpenOptions,
    };

    Archive::open(path, OpenOptions::new().read(true).write(true))?
        .append(spec.parse()?, Dir::encode(build, 9)?)?;

    Ok(())
}
pub fn unpack(source: &str, dest: &str, spec: &str) -> Result<(), Error> {
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(source, OpenOptions::new().read(true))?.unpack(dest, &spec.parse()?)?;

    Ok(())
}
