use crate::Error;

pub fn uninstall(name: &str) -> Result<(), Error> {
    use {library::database::Register, std::fs::remove_file, try_traits::default::TryDefault};

    let mut register = Register::try_default()?;

    for file in register.files(name)?.lines() {
        remove_file(dbg!(file))?;
    }

    register.remove(name)?;

    Ok(())
}
