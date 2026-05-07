use super::parser::{SipMessage, DialogKey};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub dialog_key: DialogKey,
    pub messages: Vec<SipMessage>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub state: String, // "Early", "Confirmed", "Terminated"
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

    pub fn add_message(&mut self, message: SipMessage) {
        if let Some(dialog_key) = message.get_dialog_key() {
            // Group by Call-ID only, ignore tag differences (covers both directions)
            let key = dialog_key.call_id.clone();

            let session = self.sessions.entry(key.clone()).or_insert_with(|| Session {
                dialog_key: dialog_key.clone(),
                messages: Vec::new(),
                start_time: message.timestamp.clone(),
                end_time: None,
                state: "Early".to_string(),
            });

            session.messages.push(message);

            // Sort messages by timestamp
            session.messages.sort_by(|a, b| {
                // Parse timestamps for proper sorting
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
            session.state = Self::determine_state(&session.messages);
            session.end_time = session.messages.last().map(|m| m.timestamp.clone());
        }
    }

    fn determine_state(messages: &[SipMessage]) -> String {
        // Check for BYE (termination)
        for msg in messages {
            if let Some(method) = &msg.method {
                if method == "BYE" {
                    return "Terminated".to_string();
                }
            }
        }

        // Check for 200 OK to INVITE (confirmed)
        let mut has_invite_response = false;
        for msg in messages {
            if let (Some(method), Some(code)) = (&msg.method, msg.status_code) {
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

    pub fn get_sessions(&self) -> Vec<Session> {
        self.sessions.values().cloned().collect()
    }

    pub fn get_message_by_id(&self, message_id: &str) -> Option<&SipMessage> {
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
