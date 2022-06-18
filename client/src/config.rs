use {
    serde::{Deserialize, Serialize},
    std::net::SocketAddr,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<SocketAddr>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            servers: vec![SocketAddr::from(([127, 0, 0, 1], 1337))],
        }
    }
}
