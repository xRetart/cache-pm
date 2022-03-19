use {
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{
        fmt::{self, Display, Formatter},
        num::ParseIntError,
        str::FromStr,
    },
};

#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Metadata {
    pub name: String,
    pub version: Version,
}
impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (v{})", self.name, self.version)
    }
}

// e.g. 1.3.22-alpha
#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
impl FromStr for Version {
    type Err = ParseIntError;
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
