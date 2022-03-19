mod args;

fn main() {
    use {
        args::{Args, Command},
        clap::Parser,
        library::{
            archive::{append, create, read},
            package::{Build, Metadata},
        },
    };

    let args = Args::parse();
    match args.command {
        Command::Read { path } => println!("{}", read(path).unwrap()),
        Command::Create {
            path,
            name,
            version,
        } => create(
            path,
            Metadata {
                name,
                version: version.parse().unwrap(),
            },
        )
        .unwrap(),
        Command::Append {
            path,
            specification,
            build,
        } => append(
            path,
            specification.parse().unwrap(),
            Build::archive(build, 9).unwrap(),
        )
        .unwrap(),
        Command::Unpack {
            path,
            destination,
            specification,
        } => read(path)
            .unwrap()
            .unpack(&specification.parse().unwrap(), destination)
            .unwrap(),
    }
}
