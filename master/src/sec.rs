use std::time::{Duration, Instant};

use colour::{blue_ln, green_ln, red_ln, white_ln};
use tokio::{sync::Mutex, time::sleep};

use crate::JsonForSec;

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
    url: String,
    status: Mutex<Status>,
    client: reqwest::Client,
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
        }
    }
    pub async fn get(&self) -> RecResult {
        if self.status.lock().await.status != SecStatus::Healthy {
            self.wait_for_awailability().await;
        }
        let mut ret = self.client.get(&self.url).send().await;
        while let Err(e) = ret {
            red_ln!("Error while sending GET to {}: {}", self.url, e);
            self.wait_for_awailability().await;
            ret = self.client.get(&self.url).send().await;
        }
        green_ln!("Sent GET to {}", self.url);
        ret
    }
    pub async fn post(&self, json_for_sec: JsonForSec) -> RecResult {
        if self.status.lock().await.status != SecStatus::Healthy {
            self.wait_for_awailability().await;
        }
        let mut ret = self.client.post(&self.url).json(&json_for_sec).send().await;
        while let Err(e) = ret {
            red_ln!("Error while sending POST to {}: {}", self.url, e);
            self.wait_for_awailability().await;
            ret = self.client.post(&self.url).json(&json_for_sec).send().await;
        }
        green_ln!("Sent POST with {} to {}", json_for_sec.msg, self.url);
        ret
    }
    async fn wait_for_awailability(&self) {
        white_ln!("{} requested awailability check", self.url);
        {
            let mut guard = self.status.lock().await;
            if guard.status == SecStatus::Healthy {
                guard.status = SecStatus::Suspected;
            }
        }
        while self.status.lock().await.status != SecStatus::Healthy {
            if {
                let guard = self.status.lock().await;
                guard.prev_check.elapsed() < Duration::from_secs(10)
                    && guard.status == SecStatus::Unhealthy
            } {
                let dur = Duration::from_secs(10) - self.status.lock().await.prev_check.elapsed();
                white_ln!("Sleep for {} sec when checking {}", dur.as_secs() + 1, self.url);
                sleep(dur).await;
            }
            // I wonder whether it's a good idea
            if let Ok(mut guard) = self.status.try_lock() {
                guard.status = self.update_status(&guard.status).await;
                guard.prev_check = Instant::now();
            }
        }
    }
    async fn update_status(&self, status: &SecStatus) -> SecStatus {
        blue_ln!("Checking health of {}", self.url);
        let ret = if self.check_status().await {
            SecStatus::Healthy
        } else {
            match status {
                SecStatus::Healthy => SecStatus::Suspected,
                SecStatus::Suspected | SecStatus::Unhealthy => SecStatus::Unhealthy,
            }
        };
        blue_ln!("The status of {} is {:?}", self.url, ret);
        ret
    }
    async fn check_status(&self) -> bool {
        self.client
            // .get(url.to_string() + "/health")
            .get(self.url.to_string())
            .send()
            .await
            .is_ok()
    }
}
