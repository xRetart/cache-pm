use {
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};

#[derive(PartialEq, Eq, Hash, Debug, Archive, Deserialize, Serialize)]
#[archive_attr(derive(PartialEq, Eq, Hash, CheckBytes))]
pub struct Spec {
    pub architecture: Architecture,
}
impl Display for Spec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.architecture))
    }
}
impl FromStr for Spec {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|architecture| Self { architecture })
    }
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
#[archive_attr(derive(PartialEq, Eq, Hash, CheckBytes))]
pub enum Architecture {
    X86_64,
    X86,
    ARM64,
    ARM,
    PPC64,
    PPC,
    Alpha,
    Sparc,
}
impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl FromStr for Architecture {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X86_64" => Ok(Self::X86_64),
            "X86" => Ok(Self::X86),
            "ARM64" => Ok(Self::ARM64),
            "ARM" => Ok(Self::ARM),
            "PPC64" => Ok(Self::PPC64),
            "PPC" => Ok(Self::PPC),
            "Alpha" => Ok(Self::Alpha),
            "Sparc" => Ok(Self::Sparc),
            _ => Err(()),
        }
    }
}
