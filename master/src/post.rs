use std::pin::Pin;

use colour::{magenta_ln, red_ln};
use futures::{
    future::{join_all, select_all},
    Future, FutureExt,
};
use tokio::spawn;

use crate::{sec::SecStatus, InpJsonProxy, JsonForSec, MsgVec, SecVec, QUORUM};

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

pub async fn add_message(inp: InpJsonProxy, msgs: MsgVec, secs: SecVec) -> String {
    if inp.m < 1 {
        return "Wrong m".to_string();
    }

    if QUORUM {
        let s = join_all(secs.iter().map(|sec| sec.status()))
            .await
            .into_iter()
            .filter(|status| *status == SecStatus::Healthy)
            .count();

        magenta_ln!("Found {} healthy secondaries", s);

        if inp.m - 1 > s.try_into().unwrap() {
            red_ln!("Not enough working secondaries, message is abandoned");
            return "Not enough working secondaries, message is abandoned".to_string();
        };
    }
    let msg = inp.msg.clone();

    magenta_ln!("Started sending {}!", msg);

    let id: usize;
    {
        let mut lock = msgs.lock().await;
        id = lock.len();
        lock.push(msg.clone());
    }
    let json_for_sec = JsonForSec { msg, id };

    let responces = quorum(inp.m, json_for_sec, secs).await;

    magenta_ln!("Enough secondaries reached, returning");
    for task in responces {
        spawn(task);
    }
    "Added message to the list".to_string()
}
