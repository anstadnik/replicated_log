use colour::cyan_ln;

use serde::{Deserialize, Serialize};
use tokio::spawn;

use crate::{MsgVec, SecVec, VERBOSE};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RetJson {
    pub msg: Vec<String>,
}

pub async fn get_messages(msgs: MsgVec, secs: SecVec) -> warp::reply::Json {
    if VERBOSE {
        for sec in secs.iter() {
            // Gotta move sec_ inside the closure
            let sec_ = sec.clone();
            spawn(async move { sec_.get().await });
        }
        cyan_ln!("Messages: {:?}", msgs.lock().unwrap());
    }

    let msg = msgs.lock().unwrap().clone();
    warp::reply::json(&RetJson { msg })
}
