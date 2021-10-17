use warp::{http, Filter};
use std::convert::Infallible;
use snafu::{ensure, Snafu};
// use futures::{stream, StreamExt}; // 0.3.5

use parking_lot::RwLock;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// const SEC_IPS: [&'static str; 2] = ["http://sec1:5000/msgs", "http://sec2:5001/msgs"];
const SEC_IPS: [&'static str; 2] = ["http://localhost:5000", "http://localhost:5000"];

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MessageJsonProxy {
    msg: String,
}

#[derive(Clone)]
struct Messages {
    messages: Arc<RwLock<Vec<String>>>,
}

impl Messages {
    fn new() -> Self {
        Messages {
            messages: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

/* // A normal error type, created by SNAFU
#[derive(Debug, Snafu)]
#[snafu(display("I hate rust: {}", value))]
struct CustomError {
    value: i32,
}

// We need a custom type to later extract from the `Rejection`. In
// this case, we can reuse the error type itself.
impl warp::reject::Reject for CustomError {}

// To allow using `?`, we implement a conversion from our error to
// `Rejection`
impl From<CustomError> for warp::Rejection {
    fn from(other: CustomError) -> Self {
        warp::reject::custom(other)
    }
} */

/* impl From<reqwest::error::Error> for warp::reject::Rejection {
    fn from(other: CustomError) -> Self {
        warp::reject::custom(other)
    }
} */

async fn add_message(
    item: MessageJsonProxy,
    messages: Messages,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut map = HashMap::new();
    map.insert("mes", &item.msg);

    let client = reqwest::Client::new();
    for ip in SEC_IPS {
        println!("SENDING to {}", ip);
        client.post(ip).json(&map).send().await;
    }

    messages.messages.write().push(item.msg);

    println!("{:?}", messages.messages);

    Ok(warp::reply::with_status(
        "Added message to the list",
        http::StatusCode::CREATED,
    ))
}

// async fn get_messages(messages: Messages) -> Result<impl warp::Reply, warp::Rejection> {
async fn get_messages(messages: Messages) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = Vec::new();
    let r = messages.messages.read();

    let client = reqwest::Client::new();
    for ip in SEC_IPS {
        println!("SENDING to {}", ip);
        let ret = client.get(ip).send().await;
    }

    println!("{:?}", messages.messages);

    for mes in r.iter() {
        result.push(mes);
    }

    Ok(warp::reply::json(&result))
}

fn json_body() -> impl Filter<Extract = (MessageJsonProxy,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let messages = Messages::new();
    let messages_filter = warp::any().map(move || messages.clone());

    // let client = reqwest::Client::new();
    let add_items = warp::post()
        .and(json_body())
        // .and()
        .and(messages_filter.clone())
        .and_then(add_message);


    let get_items = warp::get()
        .and(messages_filter.clone())
        .and_then(get_messages);

    let routes = add_items.or(get_items);

    warp::serve(routes).run(([0, 0, 0, 0], 7878)).await;
}
