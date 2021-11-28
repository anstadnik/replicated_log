use std::{thread::sleep, time::Duration};

use colour::{cyan_ln, green_ln, red_ln};
use serde::{Deserialize, Serialize};
use tokio::spawn;

use crate::{MsgVec, VERBOSE};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RetJson {
    pub msg: Vec<String>,
}

async fn send_get(ip: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut ret = client.get(ip).send().await;
    while let Err(e) = ret {
        red_ln!("Error while sending GET: {}", e);
        sleep(Duration::from_secs(5));
        ret = client.get(ip).send().await;
    }
    green_ln!("Sent GET to {}", ip);
    ret
}

pub async fn get_messages(msgs: MsgVec, sec_ips: [&'static str; 2]) -> warp::reply::Json {
    if VERBOSE {
        for ip in sec_ips {
            spawn(send_get(ip));
        }
        cyan_ln!("Messages: {:?}", msgs.lock().unwrap());
    }

    let msg = msgs.lock().unwrap().clone();
    warp::reply::json(&RetJson { msg })
}
