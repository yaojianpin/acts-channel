use serde::{Deserialize, Serialize};

macro_rules! unpack_value_string {
    ($value: ident, $name: expr) => {
        $value.get($name).unwrap().as_str().unwrap().to_string()
    };
}

macro_rules! unpack_value_number {
    ($value: ident, $name: expr) => {
        $value.get($name).unwrap().as_f64().unwrap()
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcInfo {
    pub id: String,
    pub name: String,
    pub mid: String,
    pub state: String,
    pub start_time: i64,
    pub end_time: i64,
    pub vars: String,
    pub timestamp: i64,
    pub tasks: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub name: String,
    pub proc_id: String,
    pub node_id: String,
    pub r#type: String,
    pub state: String,
    pub action_state: String,
    pub start_time: i64,
    pub end_time: i64,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub ver: u32,
    pub size: u32,
    pub time: i64,
    pub model: String,
}

impl From<&serde_json::Value> for ModelInfo {
    fn from(value: &serde_json::Value) -> Self {
        Self {
            id: unpack_value_string!(value, "id"),
            name: unpack_value_string!(value, "name"),
            model: unpack_value_string!(value, "model"),
            ver: unpack_value_number!(value, "ver") as u32,
            size: unpack_value_number!(value, "size") as u32,
            time: unpack_value_number!(value, "time") as i64,
        }
    }
}

impl From<&serde_json::Value> for ProcInfo {
    fn from(value: &serde_json::Value) -> Self {
        Self {
            id: unpack_value_string!(value, "id"),
            name: unpack_value_string!(value, "name"),
            mid: unpack_value_string!(value, "mid"),
            state: unpack_value_string!(value, "state"),
            start_time: unpack_value_number!(value, "start_time") as i64,
            end_time: unpack_value_number!(value, "end_time") as i64,
            vars: unpack_value_string!(value, "vars"),
            timestamp: unpack_value_number!(value, "timestamp") as i64,
            tasks: unpack_value_string!(value, "tasks"),
        }
    }
}

impl From<&serde_json::Value> for TaskInfo {
    fn from(value: &serde_json::Value) -> Self {
        Self {
            id: unpack_value_string!(value, "id"),
            name: unpack_value_string!(value, "name"),
            proc_id: unpack_value_string!(value, "proc_id"),
            node_id: unpack_value_string!(value, "node_id"),
            r#type: unpack_value_string!(value, "type"),
            state: unpack_value_string!(value, "state"),
            action_state: unpack_value_string!(value, "action_state"),
            start_time: unpack_value_number!(value, "start_time") as i64,
            end_time: unpack_value_number!(value, "end_time") as i64,
            timestamp: unpack_value_number!(value, "timestamp") as i64,
        }
    }
}
