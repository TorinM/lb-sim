use crate::backend::server::Server;

#[derive(Debug, Clone)]
struct LoadBalancer {
    pub server_list: Vec<Server>,
    current_server_index: usize,
}
impl LoadBalancer {
    pub fn new(server_list: Vec<Server>) -> Self {
        Self {
            server_list,
            current_server_index: 0
        }
    }
}
