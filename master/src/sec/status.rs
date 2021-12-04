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
                let dur = Duration::from_secs(11) - self.status.lock().await.prev_check.elapsed();
                white_ln!("Sleep for {} sec when checking {}", dur.as_secs(), self.url);
                sleep(dur).await;
            }
            // I wonder whether it's a good idea
            if let Ok(mut guard) = self.status.try_lock() {
                guard.status = self.get_new_status(&guard.status).await;
                guard.prev_check = Instant::now();
            }
        }
    }
    pub async fn recheck_status(&self) -> SecStatus {
        blue_ln!("{} requested a status recheck", self.url);
        self.status.lock().await.status = SecStatus::Suspected;
        self.status().await
    }
    pub async fn status(&self) -> SecStatus {
        let mut guard = self.status.lock().await;
        if guard.status == SecStatus::Suspected {
            blue_ln!("Status of {} is suspected, recheck", self.url);
            guard.status = self.update_status(&guard.status).await;
            blue_ln!("The new status of {} is {:?}", self.url, guard.status);
        }
        guard.status.clone()
    }
    async fn get_new_status(&self, status: &SecStatus) -> SecStatus {
        blue_ln!("Checking health of {}", self.url);
        let mut ret = self.update_status(status).await;
        if ret == SecStatus::Suspected {
            blue_ln!("Health of {} is suspected, checking again", self.url);
            ret = self.update_status(&ret).await;
        }
        blue_ln!("The status of {} is {:?}", self.url, ret);
        ret
    }
    async fn update_status(&self, status: &SecStatus) -> SecStatus {
        let health_url = self.url.to_string() + "/health";
        if self.client_timeout.get(health_url).send().await.is_ok() {
            SecStatus::Healthy
        } else {
            match status {
                SecStatus::Healthy => {
            blue_ln!("Health of {} is suspected, checking again", self.url);

                },
                SecStatus::Suspected | SecStatus::Unhealthy => SecStatus::Unhealthy,
            }
        }
    }
}
