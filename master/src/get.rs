use futures::future::join_all;

use crate::{Messages, SHOW_MESSAGES_ON_ALL_HOSTS};

pub async fn get_messages(messages: Messages, sec_ips: [&str; 2]) -> String {
    if SHOW_MESSAGES_ON_ALL_HOSTS {
        let client = reqwest::Client::new();
        let responces: Vec<_> = sec_ips.iter().map(|ip| client.get(*ip).send()).collect();
        join_all(responces).await;
    }

    println!("Messages: {:?}", messages.messages.lock().unwrap());

    "Printed messages to the command line".to_string()
    /* warp::reply::with_status(
        "Printed messages to the command line",
        http::StatusCode::CREATED,
    ) */
}
