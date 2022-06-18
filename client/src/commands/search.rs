use crate::Error;

pub fn search(part: &str) -> Result<(), Error> {
    use {try_traits::default::TryDefault, library::database::Core};

    Core::try_default()?
        .search(part)
        .and_then(Iterator::collect::<sqlite3::Result<Vec<_>>>)
        .map(|finds| {
            for find in finds {
                println!("{}", find);
            }
        })?;

    Ok(())
}
