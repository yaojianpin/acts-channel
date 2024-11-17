// mod error;
mod info;
mod message;
mod package;
// mod result;

use serde_json::Value;

pub type ActValue = Value;
// pub use error::ActError;
pub use info::{MessageInfo, ModelInfo, PackageInfo, ProcInfo, TaskInfo};
pub use message::Message;
pub use package::Package;
// pub use result::Result;
