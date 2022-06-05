use {std::net::SocketAddr, serde::{Serialize, Deserialize}};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<SocketAddr>,
}
impl Default for Config {
    fn default() -> Self {
        Self { servers: vec![SocketAddr::from(([127, 0, 0, 1], 1337))] }
    }
}
