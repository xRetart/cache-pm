use {
    crate::error,
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
    /// Searches for a packages containing `part` in the `packages` table
    /// # Errors
    /// Returns `sqlite3::Error` if preparing the sql statement failed.
    pub fn search<P: AsRef<str>>(&self, part: P) -> sqlite3::Result<NameIter> {
        self.conn
            .prepare(format!(
                "SELECT name FROM packages WHERE name LIKE '%{}%'",
                part.as_ref()
            ))
            .map(|statement| NameIter {
                cursor: statement.cursor(),
            })
    }

    /// Queries all information a package called `name` available in the `packages` table
    /// # Errors
    /// Returns `library::error::Info::SQLite3` if preparing the statement or advancing to the next
    /// row in the table failed.
    /// Returns `library::error::Info::PackageNotFound` if a package with name column `name` is not
    /// present in `packages`.
    /// Returns `library::error::Info::InvalidColumn` if the `packages` table in the database is
    /// invalid.
    pub fn info<N: AsRef<str>>(&self, name: N) -> Result<Info, error::Info> {
        let statement = format!("SELECT name, version, description, build_depend, run_depend FROM packages WHERE name = '{}'", name.as_ref());

        let mut cursor = self
            .conn
            .prepare(statement)
            .map_err(error::Info::SQLite3)?
            .cursor();
        let row = cursor
            .next()
            .map_err(error::Info::SQLite3)?
            .ok_or(error::Info::PackageNotFound)?;

        let string_at = move |n: usize| {
            row[n]
                .as_string()
                .ok_or(error::Info::InvalidColumn)
                .map(str::to_owned)
        };
        Ok(Info {
            name: string_at(0)?,
            version: string_at(1)?,
            description: string_at(2)?,
            build_depend: string_at(3)?,
            run_depend: string_at(4)?,
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

/// Information in a row of the `packages` table
pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
    pub build_depend: String,
    pub run_depend: String,
}
