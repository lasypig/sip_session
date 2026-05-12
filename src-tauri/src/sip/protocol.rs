use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
use super::parser::SipMessage;

pub type Message = SipMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub key: String,
    pub protocol: String,
    pub messages: Vec<Message>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub state: String,
}

