pub mod get;
pub mod post;

pub use get::get_messages;
pub use post::add_message;

use std::sync::{Arc, Mutex};

const VERBOSE: bool = true;

type MsgVec = Arc<Mutex<Vec<String>>>;
