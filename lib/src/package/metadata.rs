use {
    crate::error::ParseMetadata,
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{
        fmt::{self, Display, Formatter},
        num::ParseIntError,
        str::FromStr,
    },
};

/// Contains general information about a `Package`
#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Metadata {
    /// Name of the `Package`
    pub name: String,

    /// Version of the `Package`
    pub version: Version,
}
impl Display for Metadata {
    /// Pretty-prints the `Metadata` with the following format:
    /// <name> (v<version>)
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (v{})", self.name, self.version)
    }
}
impl FromStr for Metadata {
    type Err = ParseMetadata;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        const SEPARATOR: char = ':';

        raw.split_once(SEPARATOR)
            .ok_or(ParseMetadata::Format)
            .and_then(|(name, version)| {
                Ok(Self {
                    name: name.to_owned(),
                    version: version.parse()?,
                })
            })
    }
}

/// A version in the `SemVer` format
#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
impl Display for Version {
    /// Pretty-prints the `Version` with the following format:
    /// <major>.<minor>.<patch>
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
impl FromStr for Version {
    type Err = ParseIntError;

    /// Parses a `Version` from a string from the same format in that it is displayed.
    /// If a version number is omitted then all following numbers are assumed to be 0 (e.g. "2.4" = "2.4.0").
    /// # Errors
    /// Returns `std::num::ParseIntError` when `s` contains non digit characters (except
    /// the appropriate dots).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');
        let mut next = move || split.next().map_or(Ok(0), str::parse);

        Ok(Self {
            major: next()?,
            minor: next()?,
            patch: next()?,
        })
    }
}
