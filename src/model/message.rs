use crate::{WorkflowMessage, WorkflowModel};
use serde::{Deserialize, Serialize};

type Vars = serde_json::Map<String, serde_json::Value>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    /// workflow id
    pub id: String,

    /// workflow tag
    pub tag: String,

    /// workflow name
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    /// message id
    pub id: String,

    /// node name or action name
    pub name: String,

    /// task action state
    pub state: String,

    /// message type which is node kind or other message type
    pub r#type: String,

    /// message source on workflow, step and act
    pub source: String,

    /// workflow model
    pub model: Model,

    /// proc id
    pub pid: String,

    /// task id
    pub tid: String,

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

impl From<WorkflowModel> for Model {
    fn from(v: WorkflowModel) -> Self {
        Self {
            id: v.id,
            name: v.name,
            tag: v.tag,
        }
    }
}

impl From<WorkflowMessage> for Message {
    fn from(v: WorkflowMessage) -> Self {
        let inputs = crate::Vars::from_prost(&v.inputs.unwrap());
        let outputs = crate::Vars::from_prost(&v.outputs.unwrap());
        Self {
            id: v.id,
            r#type: v.r#type,
            source: v.source,
            name: v.name,
            pid: v.pid,
            tid: v.tid,
            model: v.model.unwrap().into(),
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
