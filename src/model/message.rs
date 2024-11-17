use crate::Vars;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum MessageState {
    #[default]
    None,
    Created,
    Completed,
    Submitted,
    Backed,
    Cancelled,
    Aborted,
    Skipped,
    Error,
    Removed,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    /// workflow id
    pub id: String,

    /// workflow tag
    pub tag: String,

    /// workflow name
    pub name: String,
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    /// message id
    pub id: String,

    /// task id
    pub tid: String,

    /// node name or action name
    pub name: String,

    /// task action state
    pub state: MessageState,

    /// message type
    /// msg | req
    pub r#type: String,

    // node kind
    pub source: String,

    pub model: Model,

    /// proc id
    pub pid: String,

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

    /// record the message retry times
    pub retry_times: i32,
}

impl Message {
    pub fn state(&self) -> MessageState {
        let state = self.state.clone().into();
        state
    }

    pub fn is_key(&self, key: &str) -> bool {
        self.key == key
    }

    pub fn is_state(&self, state: &str) -> bool {
        self.state == state.into()
    }

    pub fn is_type(&self, t: &str) -> bool {
        self.r#type == t
    }

    pub fn is_source(&self, t: &str) -> bool {
        self.source == t
    }

    pub fn is_tag(&self, tag: &str) -> bool {
        self.tag == tag
    }

    pub fn type_of(&self, mtype: &str) -> Option<&Self> {
        if &self.r#type == mtype {
            return Some(self);
        }
        None
    }

    pub fn tag_of(&self, tag: &str) -> Option<&Self> {
        if tag == &self.tag {
            return Some(self);
        }

        None
    }

    pub fn key_of(&self, key: &str) -> Option<&Self> {
        if key == &self.key {
            return Some(self);
        }

        None
    }

    /// workflow cost in million seconds
    pub fn cost(&self) -> i64 {
        if self.state.is_completed() {
            return self.end_time - self.start_time;
        }

        0
    }
}

impl MessageState {
    pub fn is_completed(&self) -> bool {
        match self {
            MessageState::Completed
            | MessageState::Cancelled
            | MessageState::Submitted
            | MessageState::Backed
            | MessageState::Error
            | MessageState::Skipped
            | MessageState::Aborted
            | MessageState::Removed => true,
            _ => false,
        }
    }
}

impl fmt::Display for MessageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s: String = self.into();
        f.write_str(&s)
    }
}

// impl From<TaskState> for MessageState {
//     fn from(state: TaskState) -> Self {
//         match state {
//             TaskState::None => MessageState::None,
//             TaskState::Ready | TaskState::Pending | TaskState::Running | TaskState::Interrupt => {
//                 MessageState::Created
//             }
//             TaskState::Completed => MessageState::Completed,
//             TaskState::Submitted => MessageState::Submitted,
//             TaskState::Backed => MessageState::Backed,
//             TaskState::Cancelled => MessageState::Cancelled,
//             TaskState::Error => MessageState::Error,
//             TaskState::Aborted => MessageState::Aborted,
//             TaskState::Skipped => MessageState::Skipped,
//             TaskState::Removed => MessageState::Removed,
//         }
//     }
// }

impl From<MessageState> for String {
    fn from(state: MessageState) -> Self {
        message_state_to_str(state)
    }
}

impl From<&str> for MessageState {
    fn from(str: &str) -> Self {
        str_to_message_state(str)
    }
}

impl From<String> for MessageState {
    fn from(str: String) -> Self {
        str_to_message_state(&str)
    }
}

impl From<&MessageState> for String {
    fn from(state: &MessageState) -> Self {
        message_state_to_str(state.clone())
    }
}

fn message_state_to_str(state: MessageState) -> String {
    match state {
        MessageState::None => "none".to_string(),
        MessageState::Aborted => "aborted".to_string(),
        MessageState::Backed => "backed".to_string(),
        MessageState::Cancelled => "cancelled".to_string(),
        MessageState::Completed => "completed".to_string(),
        MessageState::Created => "created".to_string(),
        MessageState::Skipped => "skipped".to_string(),
        MessageState::Submitted => "submitted".to_string(),
        MessageState::Error => "error".to_string(),
        MessageState::Removed => "removed".to_string(),
    }
}

fn str_to_message_state(s: &str) -> MessageState {
    match s {
        "aborted" => MessageState::Aborted,
        "backed" => MessageState::Backed,
        "cancelled" => MessageState::Cancelled,
        "completed" => MessageState::Completed,
        "created" => MessageState::Created,
        "skipped" => MessageState::Skipped,
        "submitted" => MessageState::Submitted,
        "error" => MessageState::Error,
        "removed" => MessageState::Removed,
        "none" | _ => MessageState::None,
    }
}
