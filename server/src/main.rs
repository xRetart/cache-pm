mod args;
mod commands;
mod error;

use std::io;
pub use {args::Args, error::Error};

fn main() {
    use {crate::error::report, quit::with_code};

    env_logger::init();

    let code = match result_main() {
        Ok(_) => 0,
        Err(e) => {
            report(e);
            1
        }
    };

    with_code(code);
}
fn result_main() -> Result<(), io::Error> {
    use {clap::Parser, commands::run};

    let args = Args::parse();
    run(args.port)
}
