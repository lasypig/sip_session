use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProtocolType {
    SIP,
    RTSP,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum SessionKey {
    SIP { call_id: String },
    RTSP { session_id: String,},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    SIP {
        inner: super::parser::SipMessage
    },
    RTSP {
        inner: super::rtsp_parser::RtspMessage
    },
}

impl Message {
    pub fn protocol(&self) -> ProtocolType {
        match self {
            Message::SIP { .. } => ProtocolType::SIP,
            Message::RTSP { .. } => ProtocolType::RTSP,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Message::SIP { inner } => &inner.id,
            Message::RTSP { inner } => &inner.id,
        }
    }

    pub fn timestamp(&self) -> &str {
        match self {
            Message::SIP { inner } => &inner.timestamp,
            Message::RTSP { inner } => &inner.timestamp,
        }
    }

    pub fn src_ip(&self) -> &str {
        match self {
            Message::SIP { inner } => &inner.src_ip,
            Message::RTSP { inner } => &inner.src_ip,
        }
    }

    pub fn dst_ip(&self) -> &str {
        match self {
            Message::SIP { inner } => &inner.dst_ip,
            Message::RTSP { inner } => &inner.dst_ip,
        }
    }

    pub fn src_port(&self) -> u16 {
        match self {
            Message::SIP { inner } => inner.src_port,
            Message::RTSP { inner } => inner.src_port,
        }
    }

    pub fn dst_port(&self) -> u16 {
        match self {
            Message::SIP { inner } => inner.dst_port,
            Message::RTSP { inner } => inner.dst_port,
        }
    }

    pub fn method(&self) -> Option<&str> {
        match self {
            Message::SIP { inner } => inner.method.as_deref(),
            Message::RTSP { inner } => inner.method.as_deref(),
        }
    }

    pub fn status_code(&self) -> Option<u16> {
        match self {
            Message::SIP { inner } => inner.status_code,
            Message::RTSP { inner } => inner.status_code,
        }
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        match self {
            Message::SIP { inner } => &inner.headers,
            Message::RTSP { inner } => &inner.headers,
        }
    }

    pub fn body(&self) -> Option<&str> {
        match self {
            Message::SIP { inner } => inner.body.as_deref(),
            Message::RTSP { inner } => inner.body.as_deref(),
        }
    }

    pub fn raw_data(&self) -> &[u8] {
        match self {
            Message::SIP { inner } => &inner.raw_data,
            Message::RTSP { inner } => &inner.raw_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub protocol: ProtocolType,
    pub key: SessionKey,
    pub messages: Vec<Message>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub state: String,
}

