use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipMessage {
    pub id: String,
    pub timestamp: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub method: Option<String>,
    pub status_code: Option<u16>,
    pub status_text: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub raw_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DialogKey {
    pub call_id: String,
    pub from_tag: String,
    pub to_tag: String,
}

impl SipMessage {
    pub fn parse(raw_data: &[u8], timestamp: &str, src_ip: &str, dst_ip: &str,
                 src_port: u16, dst_port: u16) -> Option<Self> {
        let text = String::from_utf8_lossy(raw_data).to_string();
        let lines: Vec<&str> = text.lines().collect();

        if lines.is_empty() {
            return None;
        }

        // Parse start line
        let first_line = lines[0];
        let (method, status_code, status_text) = parse_start_line(first_line);

        // Parse headers
        let mut headers = HashMap::new();
        let mut body_start = None;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() {
                body_start = Some(i + 1);
                break;
            }

            // Handle header continuation (lines starting with space/tab)
            if line.starts_with(' ') || line.starts_with('\t') {
                // Append to last header
                continue;
            }

            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Parse Body
        let body = if let Some(start) = body_start {
            if start < lines.len() {
                let body_lines: Vec<&str> = lines[start..].to_vec();
                Some(body_lines.join("\n"))
            } else {
                None
            }
        } else {
            None
        };

        // Generate unique ID
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();

        Some(SipMessage {
            id,
            timestamp: timestamp.to_string(),
            src_ip: src_ip.to_string(),
            dst_ip: dst_ip.to_string(),
            src_port,
            dst_port,
            method,
            status_code,
            status_text,
            headers,
            body,
            raw_data: raw_data.to_vec(),
        })
    }

    pub fn get_call_id(&self) -> Option<String> {
        self.headers.get("Call-ID").or_else(|| self.headers.get("call-id")).cloned()
    }

    pub fn get_from_tag(&self) -> Option<String> {
        self.headers.get("From").or_else(|| self.headers.get("from")).and_then(|f| extract_tag(f))
    }

    pub fn get_to_tag(&self) -> Option<String> {
        self.headers.get("To").or_else(|| self.headers.get("to")).and_then(|t| extract_tag(t))
    }

    pub fn get_dialog_key(&self) -> Option<DialogKey> {
        let call_id = self.get_call_id()?;
        let from_tag = self.get_from_tag()?;
        let to_tag = self.get_to_tag().unwrap_or_default();

        Some(DialogKey {
            call_id,
            from_tag,
            to_tag,
        })
    }
}

fn parse_start_line(line: &str) -> (Option<String>, Option<u16>, Option<String>) {
    if line.to_uppercase().starts_with("SIP/2.0") {
        // Status line: SIP/2.0 200 OK
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        let status_code = parts.get(1).and_then(|s| s.parse().ok());
        let status_text = parts.get(2).map(|s| s.trim().to_string());
        (None, status_code, status_text)
    } else {
        // Request line: INVITE sip:test@example.com SIP/2.0
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        let method = parts.get(0).map(|s| s.to_uppercase());
        (method, None, None)
    }
}

fn extract_tag(header_value: &str) -> Option<String> {
    // From: <sip:alice@example.com>;tag=abc123
    header_value.find("tag=").map(|pos| {
        let tag_start = pos + 4;
        let remaining = &header_value[tag_start..];
        let tag_end = remaining.find([';', ' ', '>', '\r', '\n', '\t']).unwrap_or(remaining.len());
        remaining[..tag_end].to_string()
    })
}
