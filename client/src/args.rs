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
    Install {
        name: String,

        #[clap(short, long)]
        spec: String,
    },

    /// uninstall package
    Uninstall { name: String },

    /// list all packages conforming to `pattern`
    List {
        #[clap(default_value = "")]
        pattern: String
    },

    /// get information about package called `name`
    Info { name: String },

    /// add package to owned list
    Select { name: String },

    /// remove package from owned list
    Deselect { name: String },

    /// get list of files installed by package
    Files { name: String },
}
