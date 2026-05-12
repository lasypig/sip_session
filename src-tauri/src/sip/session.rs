use super::protocol::{Message};
use super::parser::SipMessage;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub key: String,
    pub protocol: String,
    pub messages: Vec<Message>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub state: String,
}

pub struct SessionManager {
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
        }
    }

    pub fn add_sip_message(&mut self, message: SipMessage) {
        if let Some(dialog_key) = message.get_dialog_key() {
            let key = dialog_key.call_id.clone();

            let session = self.sessions.entry(key.clone()).or_insert_with(|| Session {
                key: key.clone(),
                messages: Vec::new(),
                start_time: message.timestamp.clone(),
                end_time: None,
                state: "Early".to_string(),
                protocol: "SIP".to_string(),
            });

            session.messages.push(message);

            // Sort messages by timestamp
            session.messages.sort_by(|a, b| {
                let a_parts: Vec<&str> = a.timestamp.split('.').collect();
                let b_parts: Vec<&str> = b.timestamp.split('.').collect();
                a_parts[0].cmp(&b_parts[0])
                    .then_with(|| {
                        let a_micro = a_parts.get(1).unwrap_or(&"0");
                        let b_micro = b_parts.get(1).unwrap_or(&"0");
                        a_micro.cmp(b_micro)
                    })
            });

            // Update session state
            session.state = Self::determine_sip_state(&session.messages);
            session.end_time = session.messages.last().map(|m| m.timestamp.to_string());
        }
    }

    // 辅助函数：生成双向统一的 Session Key（无方向）
    fn generate_rtsp_session_key(
        src_ip: &str,
        src_port: u16,
        dst_ip: &str,
        dst_port: u16,
    ) -> String {
        let a = format!("{}:{}", src_ip, src_port);
        let b = format!("{}:{}", dst_ip, dst_port);

        if a < b {
            format!("{}|{}", a, b)
        } else {
            format!("{}|{}", b, a)
        }
    }

    fn compare_timestamps(a: &str, b: &str) -> std::cmp::Ordering {
        let a_parts: Vec<&str> = a.split('.').collect();
        let b_parts: Vec<&str> = b.split('.').collect();

        a_parts[0].cmp(b_parts[0]).then_with(|| {
            let a_micro = a_parts.get(1).unwrap_or(&"0");
            let b_micro = b_parts.get(1).unwrap_or(&"0");
            a_micro.cmp(b_micro)
        })
    }

    pub fn add_rtsp_message(&mut self, message: SipMessage) {
        // 生成无方向的 key（核心修改）
        let key_str = Self::generate_rtsp_session_key(
            &message.src_ip,
            message.src_port,
            &message.dst_ip,
            message.dst_port,
        );

        let session = self.sessions.entry(key_str.clone()).or_insert_with(|| Session {
            key: key_str.clone(),
            messages: Vec::new(),
            start_time: message.timestamp.clone(),
            end_time: None,
            state: "Idle".to_string(),
            protocol: "SIP".to_string(),
        });

        session.messages.push(message);

        // 排序消息（建议使用独立函数，更清晰）
        session.messages.sort_by(|a, b| Self::compare_timestamps(a.timestamp.as_str(), b.timestamp.as_str()));

        // 更新会话状态和结束时间
        session.state = Self::determine_rtsp_state(&session.messages);
        session.end_time = session.messages.last().map(|m| m.timestamp.to_string());
    }

    fn determine_sip_state(messages: &[Message]) -> String {
        // Check for BYE (termination)
        for msg in messages {
            if let Some(method) = msg.method.as_ref() {
                if method == "BYE" {
                    return "Terminated".to_string();
                }
            }
        }

        // Check for 200 OK to INVITE (confirmed)
        let mut has_invite_response = false;
        for msg in messages {
            if let (Some(method), Some(code)) = (msg.method.as_ref(), msg.status_code) {
                if method == "INVITE" && code >= 200 && code < 300 {
                    has_invite_response = true;
                }
            }
        }

        if has_invite_response {
            "Confirmed".to_string()
        } else {
            "Early".to_string()
        }
    }

    fn determine_rtsp_state(messages: &[Message]) -> String {
        // RTSP state machine based on methods
        let mut has_setup = false;
        let mut has_play = false;
        let mut has_teardown = false;

        for msg in messages {
            if let Some(method) = &msg.method {
                match method.as_str() {
                    "SETUP" => has_setup = true,
                    "PLAY" => has_play = true,
                    "PAUSE" => return "Paused".to_string(),
                    "TEARDOWN" => has_teardown = true,
                    _ => {}
                }
            }
        }

        if has_teardown {
            return "Terminated".to_string();
        }

        if has_play {
            return "Playing".to_string();
        }

        if has_setup {
            return "Ready".to_string();
        }

        "Idle".to_string()
    }

    pub fn get_sessions(&self) -> Vec<Session> {
        self.sessions.values().cloned().collect()
    }

    pub fn get_message_by_id(&self, message_id: &str) -> Option<&Message> {
        for session in self.sessions.values() {
            for msg in &session.messages {
                if msg.id == message_id {
                    return Some(msg);
                }
            }
        }
        None
    }

}
