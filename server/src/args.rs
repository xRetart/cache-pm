use {clap::Parser, std::path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Port to listen to
    #[clap(long, short)]
    pub port: u16,

    /// Repository to get packages from
    #[clap(long, short)]
    pub repo: PathBuf,
}
