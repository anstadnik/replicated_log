pub mod get;
pub mod post;
pub mod sec;

pub use get::get_messages;
pub use post::add_message;
use sec::Sec;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use std::sync::Arc;

pub type MsgVec = Arc<Mutex<Vec<String>>>;
pub type SecVec = Arc<Vec<Arc<Sec>>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonForSec {
    pub msg: String,
    pub id: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InpJsonProxy {
    msg: String,
    m: usize,
}
