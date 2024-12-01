mod info;
mod message;
mod package;

use serde_json::Value;

pub type ActValue = Value;
pub use info::{MessageInfo, ModelInfo, PackageInfo, PageData, ProcInfo, TaskInfo};
pub use message::Message;
pub use package::Package;
