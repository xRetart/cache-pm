use {crate::error::Error, std::{io, net::TcpStream}};


pub fn run(port: u16) -> Result<(), Error> {
    use std::net::{TcpListener, SocketAddr};

    let listener =
        TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
            .map_err(Error::Io)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_client(&mut stream).map_err(Error::Io)?,
            Err(e) => eprintln!("unsuccesful connection ({})", e),
        }
    }

    Ok(())
}
fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    use std::io::{BufReader, BufRead};

    let mut reader = BufReader::new(stream);
    let mut spec = String::new();
    reader.read_line(&mut spec)?;

    eprintln!("requested spec {}", spec);

    Ok(())
}
