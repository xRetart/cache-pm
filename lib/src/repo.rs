use {
    crate::{error, package::metadata::Version, Package},
    std::path::Path,
};

/// Finds a package called `name` in the repository at `path`
/// # Errors
/// Returns `lib::error::Read::Io` if reading the repository at `path` failed
/// Returns `lib::error::Read` if reading the archive called `name` failed
pub fn find<P, N>(path: P, name: N, version: &Version) -> Result<Option<Package>, error::Read>
where
    P: AsRef<Path>,
    N: AsRef<str>,
{
    use std::fs::read_dir;

    let path = path.as_ref();
    let name = name.as_ref();

    let filename = format!("{}-{}.pkg", name, version);

    read_dir(path)?
        .filter_map(Result::ok)
        .find(|entry| entry.file_name() == filename.as_str())
        .map(|entry| read_pkg(entry.path()))
        .transpose()
}
fn read_pkg<P: AsRef<Path>>(path: P) -> Result<Package, error::Read> {
    use {crate::Archive, std::fs::OpenOptions};

    Archive::open(path, OpenOptions::new().read(true))?.read()
}
