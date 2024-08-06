use crate::backend::server::BackendServer;

#[derive(Debug, Clone)]
pub struct LoadBalancer {
    pub server_list: Vec<BackendServer>,
    current_server_index: usize,
    algorithm: String,
}
impl LoadBalancer {
    pub fn new(server_list: Vec<BackendServer>, algorithm: &str) -> Self {
        let lower_algorithm = algorithm.to_lowercase();
        Self {
            server_list,
            current_server_index: 0,
            algorithm: lower_algorithm,
        }
    }

    pub async fn handle_request(&mut self) {
        match self.algorithm.as_str() {
            "round-robin" => self.round_robin().await,
            alg => eprintln!("'{}' is not a valid algorithm.", alg),
        }
    }

    async fn round_robin(&mut self) {
        let server = self.server_list.get(self.current_server_index).unwrap();
        self.current_server_index = (self.current_server_index + 1) % self.server_list.len();
        server.handle_request().await;
    }
}
