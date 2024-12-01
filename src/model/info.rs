use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PageData<T> {
    pub count: usize,
    pub page_size: usize,
    pub page_num: usize,
    pub page_count: usize,
    pub rows: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcInfo {
    pub id: String,
    pub name: String,
    pub mid: String,
    pub state: String,
    pub start_time: i64,
    pub end_time: i64,
    pub timestamp: i64,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub prev: Option<String>,
    pub name: String,
    pub pid: String,
    pub nid: String,
    pub tag: String,
    pub key: String,
    pub r#type: String,
    pub state: String,
    pub data: String,
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
    pub create_time: i64,
    pub update_time: i64,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageInfo {
    pub id: String,
    pub tid: String,
    pub name: String,
    pub state: String,
    pub r#type: String,
    pub pid: String,
    pub nid: String,
    pub key: String,
    pub inputs: String,
    pub outputs: String,
    pub tag: String,
    pub create_time: i64,
    pub update_time: i64,
    pub retry_times: i32,
    pub status: String,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageInfo {
    pub id: String,
    pub name: String,
    pub data: String,
    pub size: u32,
    pub create_time: i64,
    pub update_time: i64,
    pub timestamp: i64,
}

impl From<&serde_json::Value> for ModelInfo {
    fn from(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

impl From<&serde_json::Value> for ProcInfo {
    fn from(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

impl From<&serde_json::Value> for TaskInfo {
    fn from(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

impl From<&serde_json::Value> for MessageInfo {
    fn from(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

impl From<&serde_json::Value> for PackageInfo {
    fn from(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}
