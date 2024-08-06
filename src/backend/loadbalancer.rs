use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::backend::server::Server;

#[derive(Debug, Clone)]
struct LoadBalancer {
    pub server_list: Arc<Mutex<VecDeque<Server>>>,
    current_server_index: usize,
    algorithm: String
}
impl LoadBalancer {
    pub fn new(server_list: VecDeque<Server>, algorithm: &str) -> Self {
        let lower_algorithm = algorithm.to_lowercase();
        Self {
            server_list: Arc::new(Mutex::from(server_list)),
            current_server_index: 0,
            algorithm: lower_algorithm
        }
    }
    pub async fn handle_request(&self) {
        match self.algorithm.as_str() {
            "round-robin" => self.round_robin(),
            alg => eprintln!("'{}' is not a valid algorithm.", alg) 
        }
    }

    fn round_robin(&self) {
        println!("round-robin");
    }
}
