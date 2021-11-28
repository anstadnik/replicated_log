use std::sync::{Arc, Mutex};

use master::{add_message, get_messages};
use warp::Filter;

// const SEC_IPS: [&'static str; 2] = ["http://secondary_1:5000", "http://secondary_2:5000"];
const SEC_IPS: [&'static str; 2] = ["http://localhost:5001", "http://localhost:5002"];

#[tokio::main]
async fn main() {
    let msgs = Arc::new(Mutex::new(Vec::new()));

    let messages_filter = warp::any().map(move || msgs.clone());
    let sec_ips_filter = warp::any().map(move || SEC_IPS);

    // let client = reqwest::Client::new();
    let add_items = warp::post()
        .and(warp::body::json())
        .and(messages_filter.clone())
        .and(sec_ips_filter)
        .then(add_message);

    let get_items = warp::get()
        .and(messages_filter.clone())
        .and(sec_ips_filter)
        .then(get_messages);

    let routes = add_items.or(get_items);

    warp::serve(routes).run(([0, 0, 0, 0], 7878)).await;
}
