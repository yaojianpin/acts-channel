use crate::ProtoJsonValue;
use serde_json::json;
use std::{collections::HashMap, ops::Deref};

pub struct Vars {
    pub(crate) inner: serde_json::Map<String, serde_json::Value>,
}

impl Deref for Vars {
    type Target = serde_json::Map<String, serde_json::Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::fmt::Display for Vars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = serde_json::ser::to_string_pretty(&self.inner).expect("convert vars to string");
        f.write_str(&text)
    }
}

impl Vars {
    pub fn new() -> Self {
        Self {
            inner: serde_json::Map::new(),
        }
    }

    pub fn into_inner(&self) -> serde_json::Map<String, serde_json::Value> {
        self.inner.clone()
    }

    pub fn from_prost(value: &ProtoJsonValue) -> Self {
        Self {
            inner: utils::prost_to_json(value).as_object().unwrap().clone(),
        }
    }

    pub fn from_json(value: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            inner: value.clone(),
        }
    }

    pub fn json_vars(&self) -> serde_json::Map<String, serde_json::Value> {
        self.inner.clone()
    }

    pub fn prost_vars(&self) -> crate::ProtoJsonValue {
        let mut map = HashMap::new();
        for (k, v) in self.inner.iter() {
            map.insert(k.to_string(), utils::json_to_prost(v));
        }

        crate::ProtoJsonValue {
            kind: Some(crate::proto_json_value::Kind::StructValue(crate::Struct {
                fields: map,
            })),
        }
    }

    pub fn extend(&mut self, vars: &Vars) {
        for (k, v) in vars.inner.iter() {
            self.inner.insert(k.to_string(), v.clone());
        }
    }

    pub fn value_str(&self, key: &str) -> Option<&str> {
        self.inner.get(key).map(|v| v.as_str().unwrap())
    }

    pub fn value_number(&self, key: &str) -> Option<f64> {
        self.inner.get(key).map(|v| v.as_f64().unwrap())
    }

    pub fn insert(&mut self, key: &str, value: &serde_json::Value) {
        self.inner.insert(key.to_string(), value.clone());
    }

    pub fn insert_str(&mut self, key: String, value: impl Into<String>) {
        self.inner.insert(key, json!(value.into()));
    }

    pub fn insert_number(&mut self, key: String, value: f64) {
        self.inner.insert(key, json!(value));
    }

    pub fn rm(&mut self, key: &str) {
        self.inner.remove(key);
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

mod utils {
    type JsonValue = serde_json::Value;
    type ProtoKind = crate::proto_json_value::Kind;
    use crate::{ListValue, ProtoJsonValue, Struct};
    use serde_json::json;
    use std::collections::HashMap;

    pub fn prost_to_json(v: &ProtoJsonValue) -> JsonValue {
        match &v.kind {
            Some(kind) => match kind {
                ProtoKind::NullValue(_) => JsonValue::Null,
                ProtoKind::F64Value(v) => json!(v),
                ProtoKind::I64Value(v) => json!(v),
                ProtoKind::U64Value(v) => json!(v),
                ProtoKind::StringValue(v) => json!(v),
                ProtoKind::BoolValue(v) => json!(v),
                ProtoKind::StructValue(v) => {
                    let mut obj = serde_json::Map::new();
                    for (k, v) in v.fields.iter() {
                        obj.insert(k.to_string(), prost_to_json(v));
                    }
                    JsonValue::Object(obj)
                }
                ProtoKind::ListValue(list) => {
                    let mut arr = Vec::new();
                    for v in list.values.iter() {
                        arr.push(prost_to_json(v));
                    }

                    JsonValue::Array(arr)
                }
            },
            _ => JsonValue::Null,
        }
    }

    pub fn json_to_prost(v: &JsonValue) -> ProtoJsonValue {
        match v {
            serde_json::Value::Null => ProtoJsonValue {
                kind: Some(ProtoKind::NullValue(0)),
            },
            serde_json::Value::Bool(v) => ProtoJsonValue {
                kind: Some(ProtoKind::BoolValue(v.clone())),
            },
            serde_json::Value::Number(v) => {
                if v.is_i64() {
                    return ProtoJsonValue {
                        kind: Some(ProtoKind::I64Value(v.as_i64().unwrap())),
                    };
                } else if v.is_u64() {
                    return ProtoJsonValue {
                        kind: Some(ProtoKind::U64Value(v.as_u64().unwrap())),
                    };
                }
                ProtoJsonValue {
                    kind: Some(ProtoKind::F64Value(v.as_f64().unwrap())),
                }
            }
            serde_json::Value::String(v) => ProtoJsonValue {
                kind: Some(ProtoKind::StringValue(v.clone())),
            },
            serde_json::Value::Array(arr) => {
                let mut values = Vec::new();
                for v in arr {
                    values.push(json_to_prost(v));
                }
                ProtoJsonValue {
                    kind: Some(ProtoKind::ListValue(ListValue { values })),
                }
            }
            serde_json::Value::Object(obj) => {
                let mut fields = HashMap::new();
                for (k, v) in obj {
                    fields.insert(k.to_string(), json_to_prost(v));
                }
                ProtoJsonValue {
                    kind: Some(ProtoKind::StructValue(Struct { fields })),
                }
            }
        }
    }
}
