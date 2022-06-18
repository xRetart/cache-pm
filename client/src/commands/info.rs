use crate::Error;

pub fn info(name: &str) -> Result<(), Error> {
    use {try_traits::default::TryDefault, library::database::{Core, core::Info}};

    let Info {
        name,
        version,
        description,
        build_depend,
        run_depend,
    } = Core::try_default()?.info(name)?;

    print!(
        "name: {}\nversion: {}\ndescription: {}\nbuild dependencies: {}\nrun dependencies: {}\n",
        name, version, description, build_depend, run_depend
    );

    Ok(())
}
