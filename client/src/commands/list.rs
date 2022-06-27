use crate::Error;

pub fn list(pattern: &str) -> Result<(), Error> {
    use {library::database::Core, try_traits::default::TryDefault};

    Core::try_default()?
        .list(pattern)
        .and_then(Iterator::collect::<sqlite3::Result<Vec<_>>>)
        .map(|finds| {
            for find in finds {
                println!("{}", find);
            }
        })?;

    Ok(())
}
