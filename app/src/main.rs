mod args;

fn main() {
    use {
        std::fs::OpenOptions,
        args::{Args, Command},
        clap::Parser,
        library::{
            package::{Build, Metadata},
            Archive,
        },
    };

    let args = Args::parse();
    match args.command {
        Command::Read { path } => println!("{}", Archive::open(path, OpenOptions::new().read(true)).unwrap().read().unwrap()),
        Command::Create {
            path,
            name,
            version,
        } => {
            Archive::create(
                path,
                Metadata {
                    name,
                    version: version.parse().unwrap(),
                },
            )
            .unwrap();
        }
        Command::Append {
            path,
            specification,
            build,
        } => Archive::open(path, OpenOptions::new().read(true).write(true))
            .unwrap()
            .append(
                specification.parse().unwrap(),
                Build::archive(build, 9).unwrap(),
            )
            .unwrap(),
        Command::Unpack {
            path,
            destination,
            specification,
        } => Archive::open(path, OpenOptions::new().read(true))
            .unwrap()
            .read()
            .unwrap()
            .unpack(&specification.parse().unwrap(), destination)
            .unwrap(),
    }
}
