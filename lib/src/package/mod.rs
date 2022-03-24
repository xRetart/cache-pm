pub mod dir;
pub mod metadata;
pub mod spec;

use {
    crate::error,
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{
        collections::HashMap,
        fmt::{self, Display, Formatter},
        io,
        path::Path,
    },
};
pub use {dir::Dir, metadata::Metadata, spec::Spec};

/// A `Package` contains software that can be distributed and installed
#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Package {
    /// General information about the `Package`
    pub metadata: Metadata,

    /// Map of `Spec`'s pointing to their associated `Build`
    pub distributions: HashMap<Spec, Dir>,

    /// Source code
    pub src: Dir,
}
impl Package {
    /// Creates a `Package` with `metadata` containing no `Dist`'s
    /// # Errors
    /// Returns `std::io::Erorr` when encoding the directory at path `src` failed.
    pub fn empty(metadata: Metadata, src: String) -> io::Result<Self> {
        Ok(Self {
            metadata,
            distributions: HashMap::new(),
            src: Dir::encode(src, 9)?,
        })
    }

    /// Decodes the compressed `Build` into a directory with the path `dest`
    /// # Errors
    /// Returns `lib::error::Unpacking::SpecNotFound` when the package does not contain a build associated
    /// with the specification `spec`
    pub fn unpack<P>(&self, spec: &Spec, dest: P) -> Result<(), error::Unpacking>
    where
        P: AsRef<Path>,
    {
        self.distributions
            .get(spec)
            .ok_or(error::Unpacking::SpecNotFound)
            .and_then(|build| build.decode(dest).map_err(error::Unpacking::Io))
    }
}
impl Display for Package {
    /// Pretty-prints the `Package` with the following format:
    /// <metadata>
    ///
    /// source: <source>
    /// distributions:
    ///     \[distribution\]...
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\n\nsource: {}", self.metadata, self.src.path)?;

        f.write_str("\n\ndistributions:")?;
        for spec in self.distributions.keys() {
            write!(f, "\n\t{}", spec)?;
        }

        Ok(())
    }
}
