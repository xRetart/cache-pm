use {
    crate::error::Error,
    std::{net::TcpStream, io::BufReader, str::FromStr},
};


#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}
impl Connection {
    pub fn open(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub fn reader(&self) -> BufReader<&TcpStream> {
        BufReader::new(&self.stream)
    }
    pub fn receive<I, E>(reader: &mut BufReader<&TcpStream>, conv_err: E) -> Result<I, Error>
    where
        I: FromStr,
        E: FnOnce(<I as FromStr>::Err) -> Error,
    {
        use std::io::BufRead;

        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .map_err(Error::Io)
            .and_then(|_| buf.trim().parse().map_err(conv_err))
    }
    pub fn send<D: AsRef<[u8]>>(&mut self, data: D) -> Result<(), Error> {
        use std::io::Write;

        self.stream
            .write_all(data.as_ref())
            .map_err(Error::Io)
            .map(|_| ())
    }
}
