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
    /// Prints all information in a package
    Read { path: String },

    /// Creates a new, empty package
    Create {
        path: String,
        name: String,

        #[clap(short, long)]
        vers: String,
    },

    /// Appends a build to an existing package
    Append {
        path: String,
        build: String,

        #[clap(short, long)]
        spec: String,
    },

    /// Unpacks the build of a package associated with its specification to destination
    Unpack {
        path: String,
        dest: String,

        #[clap(short, long)]
        spec: String,
    },
}
