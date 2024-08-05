use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use rand::{Rng, thread_rng};
use std::time::Duration;
use tokio::time::sleep;
use hyper::{Request, Response};
use std::convert::Infallible;
use http_body_util::Full;
use hyper::body::Bytes;



#[derive(Debug, Clone)]
pub struct Server {
    address: String,
    load_factor: Arc<AtomicUsize>,
    failure_probability: f64,
    healthy: Arc<AtomicBool>,
    processing_time: Arc<AtomicUsize>,
}
impl Server {
    pub fn new(address: String, failure_probability: f64, load_factor: Arc<AtomicUsize>, processing_time: Arc<AtomicUsize>) -> Self {
        Self {
            address,
            load_factor,
            failure_probability,
            healthy: Arc::new(AtomicBool::from(true)),
            processing_time,
        }
    }

    async fn handle_request(&self, _req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        // Simulate processing time
        let delay = self.processing_time.load(Ordering::Relaxed);
        sleep(Duration::from_millis(delay as u64)).await;
        Ok(Response::new(Full::new(Bytes::from(format!("Handled by server at {}", self.address)))))
    }


    pub async fn health_check(&self) {
        let mut rng = thread_rng();
        if rng.gen::<f64>() < self.failure_probability {
            self.healthy.store(false, Ordering::SeqCst);
            println!("Server at {} has failed.", self.address);
        } else {
            self.healthy.store(true, Ordering::SeqCst);
            println!("Server at {} is healthy.", self.address);
        }
    }
}
