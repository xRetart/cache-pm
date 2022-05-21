use {
    sqlite3::{Connection, Cursor},
    std::path::Path,
};

/// Wrapper over a `sqlite3` database located in the filesystem.
pub struct Database {
    conn: Connection,
}
impl Database {
    /// Opens the database located at `path`
    /// # Errors
    /// Returns `sqlite3::Error` if opening underlying connection failed.
    pub fn open<P: AsRef<Path>>(path: P) -> sqlite3::Result<Self> {
        use sqlite3::open;

        open(path).map(|conn| Self { conn })
    }
    /// Searches for a packages containing `part`
    /// # Errors
    /// Returns `sqlite3::Error` if preparing the sql statement failed.
    pub fn search<P: AsRef<str>>(&self, part: P) -> sqlite3::Result<NameIter> {
        self.conn
            .prepare(format!(
                "SELECT name FROM packages WHERE name LIKE '%{}%'",
                part.as_ref()
            ))
            .map(|stmt| NameIter {
                cursor: stmt.cursor(),
            })
    }
}

/// Iterator over the `name` column of the `packages` table in the database.
pub struct NameIter<'c> {
    pub cursor: Cursor<'c>,
}
impl Iterator for NameIter<'_> {
    type Item = sqlite3::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor.next() {
            Ok(row) => row.map(|row| {
                debug_assert_eq!(row.len(), 1);
                Ok(row[0].as_string().unwrap().to_owned())
            }),
            Err(e) => Some(Err(e)),
        }
    }
}
