use std::time::{Duration, Instant};

use colour::{blue_ln, white_ln};
use tokio::time::sleep;

use crate::sec::SecStatus;

use super::Sec;

impl Sec {
    pub async fn wait_for_awailability(&self) {
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
                white_ln!(
                    "Sleep for {} sec when checking {}",
                    dur.as_secs() + 1,
                    self.url
                );
                sleep(dur).await;
            }
            // I wonder whether it's a good idea
            if let Ok(mut guard) = self.status.try_lock() {
                guard.status = self.update_status(&guard.status).await;
                guard.prev_check = Instant::now();
            }
        }
    }
    pub async fn status(&self) -> SecStatus {
        let mut guard = self.status.lock().await;
        if guard.status == SecStatus::Suspected {
            blue_ln!("Status of {} is suspected, recheck", self.url);
            guard.status = self.update_status(&guard.status).await;
        }
        guard.status.clone()
    }
    pub async fn get_new_status(&self, status: &SecStatus) -> SecStatus {
        blue_ln!("Checking health of {}", self.url);
        let mut ret = self.update_status(status).await;
        if ret == SecStatus::Suspected {
            blue_ln!("Health of {} is suspected, checking again", self.url);
            ret = self.update_status(&ret).await;
        }
        blue_ln!("The status of {} is {:?}", self.url, ret);
        ret
    }
    pub async fn update_status(&self, status: &SecStatus) -> SecStatus {
        if self.check_status().await {
            SecStatus::Healthy
        } else {
            match status {
                SecStatus::Healthy => SecStatus::Suspected,
                SecStatus::Suspected | SecStatus::Unhealthy => SecStatus::Unhealthy,
            }
        }
    }
    async fn check_status(&self) -> bool {
        self.client
            .get(self.url.to_string() + "/health")
            .send()
            .await
            .is_ok()
    }
}
