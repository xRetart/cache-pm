use {crate::Error, std::path::Path};

/// installs the build in a package at path `path` with specification `spec`
/// # Errors
/// Returns `Error::Io` if opening the temporary directory fails.
/// Returns `Error::Unpack` if unpacking the archive to the temporary directory fails.
pub fn install<P, S>(path: P, spec: S) -> Result<(), Error>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    use {
        library::Archive,
        std::{env::set_current_dir, fs::OpenOptions, process::Command},
        tempfile::tempdir,
    };

    const BUILD_SCRIPT_NAME: &str = "install";

    let spec = spec.as_ref();
    let dir = tempdir().map_err(Error::Io)?;

    Archive::open(path, OpenOptions::new().read(true))
        .map_err(Error::Io)?
        .unpack(dir.path(), &spec.parse().map_err(Error::ParseSpec)?)
        .map_err(Error::Unpack)?;

    let build_path = dir.path().join(spec);
    set_current_dir(&build_path).map_err(Error::Io)?;

    Command::new(build_path.join(BUILD_SCRIPT_NAME))
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(Error::Io)?
        .success()
        .then(|| ())
        .ok_or(Error::InstallScript)
}
