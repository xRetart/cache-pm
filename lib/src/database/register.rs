use {
    crate::{error, package::metadata::Version},
    sqlite3::Connection,
    std::path::Path,
    try_traits::default::TryDefault,
};

/// Wrapper over a "register" `sqlite3` database.
pub struct Register {
    conn: Connection,
}
impl Register {
    /// Opens the register database located at `path`
    /// # Errors
    /// Returns `sqlite3::Error` if opening underlying connection failed.
    pub fn open<P: AsRef<Path>>(path: P) -> sqlite3::Result<Self> {
        use sqlite3::open;

        open(path).map(|conn| Self { conn })
    }

    /// Add entry to the `register`.
    /// # Errors
    /// Returns `sqlite3::Error` when execution of sql statement failed.
    pub fn add<N, F>(&mut self, name: N, version: &Version, files: F) -> sqlite3::Result<()>
    where
        N: AsRef<str>,
        F: AsRef<str>,
    {
        let (name, files) = (name.as_ref(), files.as_ref());

        self.conn.execute(
            format!("INSERT INTO register (name, version, files) VALUES ('{}', '{}', '{}')", name, version, files)
        )
    }

    /// Get files owned by package called `name`
    pub fn files<N: AsRef<str>>(&mut self, name: N) -> Result<String, error::Query> {
        let statement = format!("SELECT files FROM register WHERE name = '{}'", name.as_ref());

        self.conn.
            prepare(statement)?
            .cursor()
            .next()?
            .ok_or(error::Query::NotFound)?[0]
            .as_string()
            .ok_or(error::Query::InvalidColumn)
            .map(str::to_owned)
    }
}
impl TryDefault for Register {
    type Error = sqlite3::Error;
    fn try_default() -> Result<Self, Self::Error> {
        Self::open("/var/db/dist/register.db")
    }
}
