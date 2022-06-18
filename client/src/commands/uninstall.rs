use crate::Error;

pub fn uninstall(name: &str) -> Result<(), Error> {
    use {try_traits::default::TryDefault, library::database::Register, std::fs::remove_file};

    let mut register = Register::try_default()?;

    for file in register.files(name)?.lines() {
        remove_file(dbg!(file))?;
    }

    register.remove(name)?;

    Ok(())
}
