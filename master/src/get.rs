use std::collections::HashMap;

use colour::{cyan_ln, green_ln};
use tokio::spawn;

use crate::{MsgVec, VERBOSE};

async fn send_get(ip: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let ret = client.get(ip).send().await;
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

    let ret: HashMap<&str, Vec<String>> = [("m", msgs.lock().unwrap().clone())]
        .iter()
        .cloned()
        .collect();
    warp::reply::json(&ret)
}
