mod info;
mod message;
mod state;

use serde_json::Value;

pub type ActValue = Value;
pub use info::{ModelInfo, ProcInfo, TaskInfo};
pub use message::Message;
pub use state::ActionState;
