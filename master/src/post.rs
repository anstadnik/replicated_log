use std::pin::Pin;

use colour::magenta_ln;
use futures::{future::select_all, Future, FutureExt};
use tokio::spawn;

use crate::{InpJsonProxy, JsonForSec, MsgVec, SecVec, VERBOSE};

type Responces =
    Vec<Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>> + Send>>>;

async fn quorum(m: usize, json_for_sec: JsonForSec, secs: SecVec) -> Responces {
    let mut required_n = m - 1;

    let mut responces: Vec<_> = secs
        .iter()
        .map(|sec| {
            let sec_ = sec.clone();
            let json_for_sec_ = json_for_sec.clone();
            async move { sec_.post(json_for_sec_).await }.boxed()
        })
        .collect();

    while !responces.is_empty() && required_n > 0 {
        let (_val, _index, remaining) = select_all(responces).await;
        responces = remaining;
        required_n -= 1;
    }
    responces
}

pub async fn add_message(inp: InpJsonProxy, msgs: MsgVec, sec_ips: SecVec) -> String {
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
