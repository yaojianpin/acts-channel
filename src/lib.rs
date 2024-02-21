//! provides an acts client channel for acts-server

#![doc = include_str!("../README.md")]

include!("../proto/acts.grpc.rs");

mod channel;
mod vars;

pub mod model;
pub use channel::{ActsChannel, ActsOptions};
pub use vars::Vars;
