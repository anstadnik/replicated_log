use colour::{cyan_ln, yellow_ln};

use serde::{Deserialize, Serialize};
use tokio::spawn;

use crate::{MsgVec, SecVec};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RetMsgJson {
    pub msg: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RetHealthJson {
    pub url: String,
    pub status: String,
}

pub async fn get_messages(msgs: MsgVec, secs: SecVec) -> warp::reply::Json {
        for sec in secs.iter() {
            // Gotta move sec_ inside the closure
            let sec_ = sec.clone();
            spawn(async move { sec_.get().await });
        }
        cyan_ln!("Messages: {:?}", msgs.lock().await);

    let msg = msgs.lock().await.clone();
    warp::reply::json(&RetMsgJson { msg })
}

pub async fn get_health(secs: SecVec) -> warp::reply::Json {
        for sec in secs.iter() {
            yellow_ln!("Status of {} is {:?}", sec.url, sec.status().await);
        }
    
    let mut resp = Vec::new();
    for sec in secs.iter() {
        let url = sec.url.to_string();
        let status = format!("{:?}", sec.status().await);
        resp.push(RetHealthJson { url, status })
    }
    warp::reply::json(&resp)
}
