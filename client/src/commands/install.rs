use {crate::Error, std::path::Path, tempfile::TempDir};

pub fn install(name: &str, spec: &str) -> Result<(), Error> {
    use crate::commands::select;

    select(name)?;

    if name.starts_with('.') || name.starts_with('/') {
        locally(Path::new(name), spec)
    } else {
        globally(name, spec)
    }
}

/// installs the build in a package at path `path` with specification `spec`
/// # Errors
/// Returns `Error::Io` if opening the temporary directory fails.
/// Returns `Error::Unpack` if unpacking the archive to the temporary directory fails.
fn locally(path: &Path, spec: &str) -> Result<(), Error> {
    let dir = temp_dir()?;

    unpack_archive(path, dir.path(), spec)?;
    install_script(dir.path())
}

/// installs the build in a package called `name` with specification `spec`
/// # Errors
/// Returns `Error::Io` if opening the temporary directory fails.
/// Returns `Error::Unpack` if unpacking the archive to the temporary directory fails.
fn globally(name: &str, spec: &str) -> Result<(), Error> {
    use {
        library::package::Dir,
        std::{
            io::{Read, Write},
            net::{SocketAddr, TcpStream},
        },
    };

    let server = SocketAddr::from(([127, 0, 0, 1], 1337));
    let request = format!("{}:1\n{}\n", name, spec);

    let mut stream = TcpStream::connect(server).map_err(Error::Io)?;
    stream.write_all(request.as_bytes()).map_err(Error::Io)?;

    let mut data = Vec::new();
    stream.read_to_end(&mut data).map_err(Error::Io)?;

    // package was found
    if data[0] == 1 {
        let dir = temp_dir()?;

        // remove indicator byte
        data.remove(0);
        let build = Dir { data };

        build.decode(dir.path()).map_err(Error::Io)?;

        install_script(dir.path())?;

        Ok(())
    } else {
        Err(Error::PkgNotFound)
    }
}

fn temp_dir() -> Result<TempDir, Error> {
    use tempfile::tempdir;

    tempdir().map_err(Error::Io)
}
fn unpack_archive(path: &Path, dest: &Path, spec: &str) -> Result<(), Error> {
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(path, OpenOptions::new().read(true))
        .map_err(Error::Io)?
        .unpack(dest, &spec.parse().map_err(Error::ParseSpec)?)
        .map_err(Error::Unpack)
}
fn install_script(dir: &Path) -> Result<(), Error> {
    use std::{env::set_current_dir, process::Command};
    const INSTALL_SCRIPT_NAME: &str = "install";

    // `cd` into directory
    set_current_dir(dir).map_err(Error::Io)?;

    // run and wait for script to finish
    Command::new(dir.join(INSTALL_SCRIPT_NAME))
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(Error::Io)?
        .success()
        .then(|| ())
        .ok_or(Error::InstallScript)
}
