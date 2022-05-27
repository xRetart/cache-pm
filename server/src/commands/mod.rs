mod connection;

use {
    crate::error::Error,
    library::package::{Metadata, Spec},
    std::{io, path::Path, net::TcpStream},
};

pub fn run(port: u16, repo: &Path) -> Result<(), io::Error> {
    use {
        log::info,
        std::net::{SocketAddr, TcpListener},
    };

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client_thread(stream, repo),
            Err(e) => info!("unsuccesful connection: {}", e),
        }
    }

    Ok(())
}
fn handle_client_thread(stream: TcpStream, repo: &Path) {
    fn handle_client(stream: TcpStream, repo: &Path) -> Result<(Metadata, Spec, bool), Error> {
        use {connection::Connection, library::repo};

        let mut connection = Connection::open(stream);

        let mut reader = connection.reader();
        let metadata: Metadata = Connection::receive(&mut reader, Error::ParseMetadata)?;
        let spec: Spec = Connection::receive(&mut reader, Error::ParseSpec)?;

        let build = repo::find(repo, &metadata.name)
            .map_err(Error::Finding)?
            .and_then(|mut pkg| pkg.distributions.remove(&spec).map(|build| build.data));

        // first byte of response if whether or not package was found (1: found, 0: not found)
        // following is the data if found
        match build {
            Some(mut build) => {
                build.insert(0, 1_u8);
                connection.send(&build)?;

                Ok((metadata, spec, true))
            }
            None => {
                connection.send(&[0_u8])?;
                Ok((metadata, spec, false))
            },
        }
    }

    use log::info;

    let peer = stream
        .peer_addr()
        .as_ref()
        .map_or("<unknown>".to_owned(), ToString::to_string);

    match handle_client(stream, repo) {
        Ok((metadata, spec, served)) =>
            if served {
                info!("served {} with {}/{}", peer, metadata, spec)
            }
            else {
                info!("by {} requested {}/{} is not in repository", peer, metadata, spec)
            },
        Err(e) => info!("serving {} failed because: {}", peer, e),
    }
}
