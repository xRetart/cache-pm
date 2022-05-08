mod connection;

use {
    crate::error::Error,
    library::package::{Metadata, Spec},
    std::net::TcpStream,
};

pub fn run(port: u16) -> Result<(), Error> {
    use std::net::{SocketAddr, TcpListener};

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).map_err(Error::Io)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream)?,
            Err(e) => eprintln!("unsuccesful connection ({})", e),
        }
    }

    Ok(())
}
fn handle_client(stream: TcpStream) -> Result<(), Error> {
    use {connection::Connection, library::repo};

    let mut connection = Connection::open(stream);

    let mut reader = connection.reader();
    let metadata: Metadata = Connection::receive(&mut reader, Error::ParseMetadata)?;
    let spec: Spec = Connection::receive(&mut reader, Error::ParseSpec)?;

    let build = repo::find("/home/main/test-repo", metadata.name)
        .map_err(Error::Finding)?
        .and_then(|mut pkg| pkg.distributions.remove(&spec).map(|build| build.data));

    // first byte of response if whether or not package was found (1: found, 0: not found)
    // following is the data if found
    match build {
        Some(mut build) => {
            build.insert(0, 1_u8);
            connection.send(build)
        }
        None => connection.send(&[0_u8]),
    }
}
