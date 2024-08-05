use crate::backend::server::Server;

#[derive(Debug, Clone)]
pub enum Algorithms {
    RoundRobin
}


#[derive(Debug, Clone)]
struct LoadBalancer {
    pub server_list: Vec<Server>,
    current_server_index: usize,
    algorithm: Algorithms,
}
impl LoadBalancer {
    pub fn new(server_list: Vec<Server>, algorithm: Algorithms) -> Self {
        Self {
            server_list,
            current_server_index: 0,
            algorithm,
        }
    }
    pub fn start(&self)
    {

    }
}
