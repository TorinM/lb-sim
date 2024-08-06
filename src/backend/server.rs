use rand::{thread_rng, Rng};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BackendServer {
    address: SocketAddr,
    failure_probability: f64,
    is_healthy: bool,
    processing_time_ms: u64,
}
impl BackendServer {
    pub fn new(
        address: SocketAddr,
        failure_probability: f64,
        processing_time_ms: u64,
    ) -> Self {
        Self {
            address,
            failure_probability,
            is_healthy: true,
            processing_time_ms,
        }
    }
    
    async fn handle_request(&self) {
        sleep(Duration::from_millis(self.processing_time_ms));
        println!("Request processed at server: {}", self.address.to_string());
    }

    pub async fn health_check(&mut self) {
        let mut rng = thread_rng();
        if rng.gen::<f64>() < self.failure_probability {
            self.is_healthy = false;
            println!("Server at {} has failed.", self.address);
        } else {
            self.is_healthy = true;
            println!("Server at {} is healthy.", self.address);
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
           
        }
    }
}

