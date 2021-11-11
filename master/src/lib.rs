pub mod get;
pub mod post;

pub use get::get_messages;
pub use post::add_message;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const SHOW_MESSAGES_ON_ALL_HOSTS: bool = true;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageJsonProxy {
    pub msg: String,
    pub m: usize,
}

#[derive(Debug, Clone)]
pub struct Messages {
    pub messages: Arc<Mutex<Vec<String>>>,
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
