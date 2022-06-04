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
        commands::{append, create, read, unpack},
    };

    let args = Args::parse();
    match args.command {
        Command::Read { path } => read(&path),
        Command::Create { path, name, vers } => create(&path, name, &vers),
        Command::Append { path, spec, build } => append(&path, build, &spec),
        Command::Unpack { path, dest, spec } => unpack(&path, &dest, &spec),
    }
}
