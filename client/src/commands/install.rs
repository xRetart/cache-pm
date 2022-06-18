use {crate::{Config, Error}, library::package::metadata::Version, std::path::Path, tempfile::TempDir};

pub fn install(name: &str, spec: &str, config: &Config) -> Result<(), Error> {
    use crate::commands::select;

    select(name)?;

    if name.starts_with('.') || name.starts_with('/') {
        locally(Path::new(name), spec)
    } else {
        globally(name, spec, config)
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
fn globally(name: &str, spec: &str, config: &Config) -> Result<(), Error> {
    use {
        library::{database::Core, package::Dir},
        std::{
            io::{Read, Write},
            net::TcpStream,
        },
        try_traits::default::TryDefault,
    };

    let version = Core::try_default()?.newest(name)?;

    let server = config.servers[0];
    let request = format!("{}:{}\n{}\n", name, version, spec);

    let mut stream = TcpStream::connect(server)?;
    stream.write_all(request.as_bytes())?;

    let mut data = Vec::new();
    stream.read_to_end(&mut data)?;

    // package was found
    if data[0] == 1 {
        let dir = temp_dir()?;
        let path = dir.path();

        // remove indicator byte
        data.remove(0);
        let build = Dir { data };

        build.decode(path)?;
        register(path, name, &version)?;
        install_script(path)?;

        Ok(())
    } else {
        Err(Error::PkgNotFound)
    }
}

fn temp_dir() -> Result<TempDir, Error> {
    use tempfile::tempdir;

    tempdir().map_err(|e| e.into())
}
fn unpack_archive(path: &Path, dest: &Path, spec: &str) -> Result<(), Error> {
    use {library::Archive, std::fs::OpenOptions};

    Archive::open(path, OpenOptions::new().read(true))?
        .unpack(dest, &spec.parse()?)?;

    Ok(())
}
fn register(dir: &Path, name: &str, version: &Version) -> Result<(), Error> {
    use {try_traits::default::TryDefault, library::database::Register, std::fs::read_to_string};


    const FILENAME_REGISTER: &str = "register";

    let files = read_to_string(dir.join(FILENAME_REGISTER))?;

    Register::try_default()?
        .add(name, version, files)?;

    Ok(())
}
fn install_script(dir: &Path) -> Result<(), Error> {
    use std::{env::set_current_dir, process::Command};
    const FILENAME_INSTALL: &str = "install";

    // `cd` into directory
    set_current_dir(dir)?;

    // run and wait for script to finish
    Command::new(dir.join(FILENAME_INSTALL))
        .spawn()
        .and_then(|mut child| child.wait())?
        .success()
        .then(|| ())
        .ok_or(Error::InstallScript)
}
