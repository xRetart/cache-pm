mod connection;


use {
    crate::error::Error,
    library::package::{Metadata, Spec},
    std::net::TcpStream,
};


pub fn run(port: u16) -> Result<(), Error> {
    use std::net::{TcpListener, SocketAddr};

    let listener =
        TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
            .map_err(Error::Io)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream)?,
            Err(e) => eprintln!("unsuccesful connection ({})", e),
        }
    }

    Ok(())
}
fn handle_client(stream: TcpStream) -> Result<(), Error> {
    use {library::repo, connection::Connection};

    let mut connection = Connection::open(stream);

    let mut reader = connection.reader();
    let metadata: Metadata = Connection::receive(&mut reader, Error::ParseMetadata)?;
    let spec: Spec = Connection::receive(&mut reader, Error::ParseSpec)?;

    let build =
        repo::find("/home/main/test-repo", metadata.name)
            .map_err(Error::Finding)?
            .and_then(|pkg| pkg.distributions.get(&spec).map(|build| build.data().to_owned()));

    connection.send(build)
}
