use std::sync::{Arc, Mutex};

use master::{add_message, get_messages, sec::Sec, SecVec};
use warp::Filter;

// const SEC_IPS: [&'static str; 2] = [Sec::new("http://secondary_1:5000"), Sec::new("http://secondary_2:5000")];
const SEC_IPS: [&'static str; 2] = ["http://localhost:5001", "http://localhost:5002"];

#[tokio::main]
async fn main() {
    let msgs = Arc::new(Mutex::new(Vec::new()));
    let secs: SecVec = Arc::new(
        SEC_IPS
            .into_iter()
            .map(|ip| Arc::new(Sec::new(ip)))
            .collect(),
    );

    let messages_filter = warp::any().map(move || msgs.clone());
    let secs_filter = warp::any().map(move || secs.clone());

    let add_items = warp::post()
        .and(warp::body::json())
        .and(messages_filter.clone())
        .and(secs_filter.clone())
        .then(add_message);

    let get_items = warp::get()
        .and(messages_filter.clone())
        .and(secs_filter.clone())
        .then(get_messages);

    let routes = add_items.or(get_items);

    warp::serve(routes).run(([0, 0, 0, 0], 7878)).await;
}
