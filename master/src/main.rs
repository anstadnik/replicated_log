use std::sync::Arc;

use master::{SecVec, add_message, get::get_health, get_messages, sec::Sec};
use tokio::sync::Mutex;
use warp::Filter;

const SEC_URLS: [&'static str; 2] = ["http://secondary_1:5000", "http://secondary_2:5000"];
// const SEC_URLS: [&'static str; 2] = ["http://localhost:5001", "http://localhost:5002"];

#[tokio::main]
async fn main() {
    let msgs = Arc::new(Mutex::new(Vec::new()));
    let secs: SecVec = Arc::new(
        SEC_URLS
            .into_iter()
            .map(|url| Arc::new(Sec::new(url)))
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

    let get_health = warp::get()
        .and(warp::path("health"))
        .and(secs_filter.clone())
        .then(get_health);

    let routes = add_items.or(get_health).or(get_items);

    warp::serve(routes).run(([0, 0, 0, 0], 7878)).await;
}
