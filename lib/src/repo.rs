use {crate::{error, Package}, std::path::Path};

/// Finds a package called `name` in the repository at `path`
/// # Errors
/// Returns `lib::error::Read::Io` if reading the repository at `path` failed
/// Returns `lib::error::Read` if reading the archive called `name` failed
pub fn find<P, N>(path: P, name: N) -> Result<Option<Package>, error::Read>
where
    P: AsRef<Path>,
    N: AsRef<str>,
{
    use std::fs::read_dir;

    let path = path.as_ref();
    let name = name.as_ref();

    read_dir(path)
        .map_err(error::Read::Io)?
        .filter_map(Result::ok)
        .find(|entry| entry.file_name() == name)
        .map(|entry| read_pkg(entry.path()))
        .transpose()
}
fn read_pkg<P: AsRef<Path>>(path: P) -> Result<Package, error::Read> {
    use {std::fs::OpenOptions, crate::Archive};

    Archive::open(path, OpenOptions::new().read(true))
        .map_err(error::Read::Io)
        .and_then(|mut archive| archive.read())
}
