mod backend;

const NUM_SERVERS: usize = 5;

#[tokio::main]
async fn main() {
    let mut server_pool: Vec<backend::server::BackendServer> = Vec::with_capacity(NUM_SERVERS);

    for i in 0..NUM_SERVERS {
        let last_quartet = 1 + i;
        let addr = format!("127.0.0.{}", last_quartet);
        server_pool.push(backend::server::BackendServer::new(
            addr,
            0.1,
            1000,
            ));
    }

    let mut lb = backend::loadbalancer::LoadBalancer::new(server_pool, "round-robin");

    loop {
        lb.handle_request().await;
    }
}
