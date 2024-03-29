use std::time::{Instant, Duration};

use tokio::sync::Mutex;

pub mod send;
mod status;

type RecResult = Result<reqwest::Response, reqwest::Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum SecStatus {
    Healthy,
    Suspected,
    Unhealthy,
}

#[derive(Debug)]
struct Status {
    status: SecStatus,
    prev_check: Instant,
}

#[derive(Debug)]
pub struct Sec {
    pub url: String,
    status: Mutex<Status>,
    client: reqwest::Client,
    client_timeout: reqwest::Client,
}

impl Sec {
    pub fn new(url: &str) -> Sec {
        Sec {
            url: url.to_string(),
            status: Mutex::new(Status {
                status: SecStatus::Suspected,
                prev_check: Instant::now(),
            }),
            client: reqwest::Client::new(),
            client_timeout: reqwest::ClientBuilder::new().timeout(Duration::from_secs(6)).build().unwrap(),
        }
    }
}
