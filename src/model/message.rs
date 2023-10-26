use crate::WorkflowMessage;
use serde::{Deserialize, Serialize};
type Vars = serde_json::Map<String, serde_json::Value>;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    /// task id
    pub id: String,

    /// node name or action name
    pub name: String,

    /// task action state
    pub state: String,

    /// message type which is node kind or other message type
    pub r#type: String,

    /// workflow id
    pub model_id: String,

    /// workflow tag
    pub model_tag: String,

    /// workflow name
    pub model_name: String,

    /// proc id
    pub proc_id: String,

    /// nodeId or specific message key
    pub key: String,

    /// from the task inputs
    pub inputs: Vars,

    /// set the outputs vars when complete the action
    pub outputs: Vars,

    /// tag to distinguish different message
    /// it is from node tag or group tag
    pub tag: String,

    /// task start time in million second
    pub start_time: i64,

    /// task end time in million second
    pub end_time: i64,
}

impl From<WorkflowMessage> for Message {
    fn from(v: WorkflowMessage) -> Self {
        let inputs = crate::Vars::from_prost(&v.inputs.unwrap());
        let outputs = crate::Vars::from_prost(&v.outputs.unwrap());
        Self {
            id: v.id,
            r#type: v.r#type,
            name: v.name,
            proc_id: v.proc_id,
            model_id: v.model_id,
            model_name: v.model_name,
            model_tag: v.model_tag,
            key: v.key,
            state: v.state,
            tag: v.tag,
            start_time: v.start_time,
            end_time: v.end_time,
            inputs: inputs.json_vars(),
            outputs: outputs.json_vars(),
        }
    }
}
