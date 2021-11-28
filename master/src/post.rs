use colour::{green_ln, magenta_ln, red_ln};
use futures::{future::select_all, Future, FutureExt};
use tokio::spawn;

use crate::{MsgVec, VERBOSE};
use serde::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};

type RecResult = Result<reqwest::Response, reqwest::Error>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InpJsonProxy {
    pub msg: String,
    pub m: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonForSec {
    pub msg: String,
    pub id: usize,
}

async fn quorum<'a>(
    m: usize,
    json_for_sec: JsonForSec,
    sec_ips: [&'static str; 2],
) -> Vec<std::pin::Pin<Box<dyn Future<Output = RecResult> + Send + 'a>>> {
    let mut required_n = m - 1;
    let mut responces: Vec<_> = sec_ips
        .iter()
        .map(|ip| send_msg(*ip, json_for_sec.clone()).boxed())
        .collect();
    while !responces.is_empty() && required_n > 0 {
        let (_val, _index, remaining) = select_all(responces).await;
        responces = remaining;
        required_n -= 1;
    }
    responces
}

async fn send_msg(ip: &str, json_for_sec: JsonForSec) -> RecResult {
    let client = reqwest::Client::new();
    let mut ret = client.post(ip).json(&json_for_sec).send().await;
    while let Err(e) = ret {
        red_ln!("Error while sending POST: {}", e);
        sleep(Duration::from_secs(5));
        ret = client.post(ip).json(&json_for_sec).send().await;
    }
    green_ln!("Sent {} to {}", json_for_sec.msg, ip);
    ret
}

pub async fn add_message(inp: InpJsonProxy, msgs: MsgVec, sec_ips: [&'static str; 2]) -> String {
    let msg = inp.msg.clone();

    if VERBOSE {
        magenta_ln!("Started sending {}!", msg);
    }

    let id: usize;
    {
        let mut lock = msgs.lock().unwrap();
        id = lock.len();
        lock.push(msg.clone());
    }
    let json_for_sec = JsonForSec { msg, id };

    let responces = quorum(inp.m, json_for_sec, sec_ips).await;

    if VERBOSE {
        magenta_ln!("Enough secondaries reached, returning");
    }
    for task in responces {
        spawn(task);
    }
    "Added message to the list".to_string()
}
