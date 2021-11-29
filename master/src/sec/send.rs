use colour::{green_ln, red_ln};

use crate::{sec::SecStatus, JsonForSec};

use super::{RecResult, Sec};

impl Sec {
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
}
