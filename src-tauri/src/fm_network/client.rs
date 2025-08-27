use std::net::SocketAddr;

#[derive(Hash)]
pub struct ClientStatus {
    pub address: SocketAddr,
    pub last_heartbeat: std::time::Instant,
}

impl ClientStatus {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            last_heartbeat: std::time::Instant::now(),
        }
    }

    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = std::time::Instant::now();
    }

    pub fn is_active(&self) -> bool {
        self.last_heartbeat.elapsed().as_secs() < 5
    }
}

impl PartialEq for ClientStatus {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for ClientStatus {}
