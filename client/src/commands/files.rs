use crate::Error;

pub fn files(name: &str) -> Result<(), Error> {
    use {library::database::Register, try_traits::default::TryDefault};

    print!("{}", Register::try_default()?.files(name)?);

    Ok(())
}
