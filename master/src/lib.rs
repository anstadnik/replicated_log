pub mod get;
pub mod post;

pub use get::get_messages;
pub use post::add_message;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const VERBOSE: bool = true;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MsgJsonProxy {
    pub msg: String,
    pub m: usize,
}

type MsgVec = Arc<Mutex<Vec<String>>>;
