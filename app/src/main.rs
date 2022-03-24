mod args;
mod commands;
mod error;

pub use error::Error;

fn main() {
    use quit::with_code;

    let code = match result_main() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            1
        }
    };

    with_code(code);
}
fn result_main() -> Result<(), Error> {
    use {
        args::{Args, Command},
        clap::Parser,
        commands::{append, create, extract, read, unpack},
    };

    let args = Args::parse();
    match args.command {
        Command::Read { path } => read(path),
        Command::Create {
            path,
            src,
            name,
            vers,
        } => create(path, src, name, vers),
        Command::Append { path, spec, build } => append(path, spec, build),
        Command::Extract { path } => extract(path),
        Command::Unpack { path, dest, spec } => unpack(path, dest, spec),
    }
}
