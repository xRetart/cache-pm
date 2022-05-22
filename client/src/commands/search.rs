use crate::Error;

pub fn search(part: &str) -> Result<(), Error> {
    use library::database::Database;

    let db = Database::open("/var/db/dist-repos/core.db").map_err(Error::SQLite3)?;
    db.search(part)
        .and_then(Iterator::collect::<sqlite3::Result<Vec<_>>>)
        .map(|finds| {
            for find in finds {
                println!("{}", find);
            }
        })
        .map_err(Error::SQLite3)
}
