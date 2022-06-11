use crate::Error;

pub fn info(name: &str) -> Result<(), Error> {
    use library::database::{core, Info};

    let Info {
        name,
        version,
        description,
        build_depend,
        run_depend,
    } = core()
        .map_err(Error::SQLite3)?
        .info(name)
        .map_err(Error::Info)?;

    print!(
        "name: {}\nversion: {}\ndescription: {}\nbuild dependencies: {}\nrun dependencies: {}\n",
        name, version, description, build_depend, run_depend
    );

    Ok(())
}
