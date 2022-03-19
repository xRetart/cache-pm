pub mod build;
pub mod metadata;
pub mod spec;

use {
    crate::error,
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{
        collections::HashMap,
        fmt::{self, Display, Formatter},
        path::Path,
    },
};
pub use {build::Build, metadata::Metadata, spec::Spec};

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Package {
    pub metadata: Metadata,
    pub distributions: HashMap<Spec, Build>,
}
impl Package {
    #[must_use]
    pub fn empty(metadata: Metadata) -> Self {
        Self {
            metadata,
            distributions: HashMap::new(),
        }
    }
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.metadata)?;

        f.write_str("\n\ndistributions:")?;
        for spec in self.distributions.keys() {
            write!(f, "\n\t{}", spec)?;
        }

        Ok(())
    }
}
