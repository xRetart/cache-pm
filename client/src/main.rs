mod args;
mod config;
mod commands;
mod error;

pub use {config::Config, error::Error};

const APP_NAME: &str = "dist";

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
        commands::{deselect, info, install, search, select},
        confy::load,
    };

    let config: Config = load(APP_NAME).map_err(Error::Confy)?;

    let args = Args::parse();
    match args.command {
        Command::Install { name, spec } => install(&name, &spec, &config),
        Command::Search { part } => search(&part),
        Command::Info { name } => info(&name),
        Command::Select { name } => select(&name),
        Command::Deselect { name } => deselect(&name),
    }
}
