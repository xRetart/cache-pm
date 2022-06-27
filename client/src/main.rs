mod args;
mod commands;
mod config;
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
        commands::{deselect, files, info, install, list, select, uninstall},
        confy::load,
    };

    let config: Config = load(APP_NAME)?;

    let args = Args::parse();
    match args.command {
        Command::Install { name, spec } => install(&name, &spec, &config),
        Command::Uninstall { name } => uninstall(&name),
        Command::List { pattern } => list(&pattern),
        Command::Info { name } => info(&name),
        Command::Select { name } => select(&name),
        Command::Deselect { name } => deselect(&name),
        Command::Files { name } => files(&name),
    }
}
