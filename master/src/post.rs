use futures::future::{join_all, select_all};

pub use crate::{MessageJsonProxy, Messages};
use std::collections::HashMap;

pub async fn add_message(item: MessageJsonProxy, messages: Messages, sec_ips: [&str; 2]) -> String {
    let map: HashMap<&str, String> = [("msg", item.msg), ("m", item.m.to_string())]
        .iter()
        .cloned()
        .collect();

    println!(
        "Started sending {}!",
        map.get("msg").unwrap().to_owned().to_string()
    );

    let mut required_n = item.m - 1;
    let client = reqwest::Client::new();
    let mut responces: Vec<_> = sec_ips
        .iter()
        .map(|ip| client.post(*ip).json(&map).send())
        .collect();
    while !responces.is_empty() && required_n > 0{
        let (val, index, remaining) = select_all(responces).await;
        dbg!(val);
        dbg!(index);
        responces = remaining;
        required_n -= 1;
    }

    messages
        .messages
        .lock()
        .unwrap()
        .push(map.get("msg").unwrap().to_owned().to_string());

    println!("Added a message {:?}", messages.messages);

    join_all(responces).await;

    "Added message to the list".to_string()
    // TODO: I could add status codes <11-11-21, astadnik> //
}
