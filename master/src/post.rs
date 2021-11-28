use colour::{green_ln, magenta_ln};
use futures::{future::select_all, Future, FutureExt};
use tokio::spawn;

pub use crate::MsgJsonProxy;
use crate::{MsgVec, VERBOSE};
use std::collections::HashMap;

type RecResult = Result<reqwest::Response, reqwest::Error>;

async fn quorum<'a>(
    m: usize,
    map: HashMap<&'a str, String>,
    sec_ips: [&'static str; 2],
) -> Vec<std::pin::Pin<Box<dyn Future<Output = RecResult> + Send + 'a>>> {
    let mut required_n = m - 1;
    let mut responces: Vec<_> = sec_ips
        .iter()
        .map(|ip| send_msg(*ip, map.clone()).boxed())
        .collect();
    while !responces.is_empty() && required_n > 0 {
        let (_val, _index, remaining) = select_all(responces).await;
        responces = remaining;
        required_n -= 1;
    }
    responces
}

async fn send_msg(ip: &str, map: HashMap<&str, String>) -> RecResult {
    let client = reqwest::Client::new();
    let ret = client.post(ip).json(&map).send().await;
    green_ln!("Sent {} to {}", map.get("msg").unwrap(), ip);
    ret
}

pub async fn add_message(inp: MsgJsonProxy, msgs: MsgVec, sec_ips: [&'static str; 2]) -> String {
    let msg = inp.msg.clone();
    let map: HashMap<&str, String> = [("msg", msg.clone()), ("m", inp.m.to_string())]
        .iter()
        .cloned()
        .collect();

    if VERBOSE {
        magenta_ln!("Started sending {}!", msg);
    }

    let responces = quorum(inp.m, map, sec_ips).await;

    msgs.lock().unwrap().push(msg);

    if VERBOSE {
        magenta_ln!("Enough secondaries reached, returning");
    }
    for task in responces {
        spawn(task);
    }
    "Added message to the list".to_string()
}
