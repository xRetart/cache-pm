mod args;
mod commands;
mod error;

use error::Error;

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
        commands::install,
    };

    let args = Args::parse();
    match args.command {
        Command::Install { name, spec } => install(name, spec),
    }
}
