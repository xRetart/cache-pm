use crate::Error;

pub fn files(name: &str) -> Result<(), Error> {
    use {try_traits::default::TryDefault, library::database::Register};

    print!("{}", Register::try_default()?.files(name)?);

    Ok(())
}
