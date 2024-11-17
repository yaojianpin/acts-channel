use core::fmt;
use serde::{Deserialize, Serialize};
use std::{io::ErrorKind, string::FromUtf8Error};
use thiserror::Error;

use super::Result;
use crate::Vars;

#[derive(Deserialize, Serialize, Error, Debug, Clone, PartialEq)]
pub enum ActError {
    #[error("{0}")]
    Convert(String),

    #[error("{0}")]
    Script(String),

    #[error("ecode: {ecode}, message: {message}")]
    Exception { ecode: String, message: String },

    #[error("{0}")]
    Model(String),

    #[error("{0}")]
    Runtime(String),

    #[error("{0}")]
    Adapter(String),

    #[error("{0}")]
    Store(String),

    #[error("{0}")]
    Action(String),

    #[error("{0}")]
    IoError(String),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    #[serde(default)]
    pub ecode: String,
    #[serde(default)]
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = serde_json::to_string(self).unwrap();
        f.write_str(&text)
    }
}

impl Error {
    pub fn new(message: &str, ecode: &str) -> Self {
        Self {
            message: message.to_string(),
            ecode: ecode.to_string(),
        }
    }

    pub fn from_var(value: &Vars) -> Result<Self> {
        serde_json::from_value::<Self>(value.clone().into()).map_err(|err| err.into())
    }
}

impl Into<String> for ActError {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<Error> for ActError {
    fn into(self) -> Error {
        match self {
            ActError::Exception { ecode, message } => Error { ecode, message },
            err => Error {
                ecode: "".to_string(),
                message: err.to_string(),
            },
        }
    }
}

impl From<std::io::Error> for ActError {
    fn from(error: std::io::Error) -> Self {
        ActError::IoError(error.to_string())
    }
}

impl Into<std::io::Error> for ActError {
    fn into(self) -> std::io::Error {
        std::io::Error::new(ErrorKind::Other, self.to_string())
    }
}

impl From<FromUtf8Error> for ActError {
    fn from(_: FromUtf8Error) -> Self {
        ActError::Runtime("Error with utf-8 string convert".to_string())
    }
}

impl From<serde_json::Error> for ActError {
    fn from(error: serde_json::Error) -> Self {
        ActError::Convert(error.to_string())
    }
}
