use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// doc
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// install package
    Install { name: String, spec: String },

    /// search for a package containing `part`
    Search { part: String },

    /// get information about package called `name`
    Info { name: String },
}
