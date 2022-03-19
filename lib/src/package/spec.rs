use {
    bytecheck::CheckBytes,
    num_derive::FromPrimitive,
    rkyv::{Archive, Deserialize, Serialize},
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};

#[derive(PartialEq, Eq, Hash, Debug, Archive, Deserialize, Serialize)]
#[archive_attr(derive(PartialEq, Eq, Hash, CheckBytes))]
pub struct Spec {
    pub arch: u8,
}
impl Display for Spec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use num_traits::FromPrimitive;
        write!(f, "{}", Arch::from_u8(self.arch).unwrap())
    }
}
impl FromStr for Spec {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Arch>().map(|arch| Self { arch: arch as u8 })
    }
}

#[derive(Archive, Deserialize, Serialize, FromPrimitive, PartialEq, Eq, Hash, Debug)]
#[archive_attr(derive(CheckBytes, PartialEq, Eq, Hash))]
pub enum Arch {
    X86_64 = 1,
    X86 = 2,
    ARM64 = 3,
    ARM = 4,
    PPC64 = 5,
    PPC = 6,
    Sparc = 7,
}
impl Display for Arch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl FromStr for Arch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X86_64" => Ok(Self::X86_64),
            "X86" => Ok(Self::X86),
            "ARM64" => Ok(Self::ARM64),
            "ARM" => Ok(Self::ARM),
            "PPC64" => Ok(Self::PPC64),
            "PPC" => Ok(Self::PPC),
            "Sparc" => Ok(Self::Sparc),
            _ => Err(()),
        }
    }
}
