pub mod archive;
pub mod error;
pub mod package;

pub use {archive::Archive, package::Package};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
