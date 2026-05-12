use tauri::State;
use crate::AppState;
use crate::sip::{pcap_reader, parser, rtsp_parser, session, protocol};

#[tauri::command]
pub async fn open_pcap_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<Vec<protocol::Message>, String> {
    eprintln!("Opening pcap file: {}", path);

    // Read pcap file
    let packets = pcap_reader::read_pcap_file(std::path::Path::new(&path))?;
    eprintln!("Total packets read: {}", packets.len());

    let mut manager = session::SessionManager::new();
    let mut messages = Vec::new();

    for packet in packets {
        eprintln!("Processing packet: protocol={}, src_ip:{}:{}, dst_ip:{}:{}",
            packet.protocol_type, packet.src_ip, packet.src_port, packet.dst_ip, packet.dst_port);

        match packet.protocol_type.as_str() {
            "SIP" => {
                if let Some(msg) = parser::SipMessage::parse(
                    &packet.payload,
                    &packet.timestamp,
                    &packet.src_ip,
                    &packet.dst_ip,
                    packet.src_port,
                    packet.dst_port,
                ) {
                    let debug_info = msg.method.clone().or(msg.status_code.map(|c| c.to_string()));
                    eprintln!("Parsed SIP message: {:?}", debug_info);
                    manager.add_sip_message(msg.clone());
                    messages.push(protocol::Message::SIP { inner: msg });
                }
            }
            "RTSP" => {
                if let Some(msg) = rtsp_parser::RtspMessage::parse(
                    &packet.payload,
                    &packet.timestamp,
                    &packet.src_ip,
                    &packet.dst_ip,
                    packet.src_port,
                    packet.dst_port,
                ) {
                    let debug_info = msg.method.clone().or(msg.status_code.map(|c| c.to_string()));
                    eprintln!("Parsed RTSP message: {:?}", debug_info);
                    manager.add_rtsp_message(msg.clone());
                    messages.push(protocol::Message::RTSP { inner: msg });
                }
            }
            _ => {
                eprintln!("Unknown protocol type: {}", packet.protocol_type);
            }
        }
    }

    // Save session manager to state
    let mut state_guard = state.session_manager.lock().unwrap();
    *state_guard = Some(manager);

    Ok(messages)
}

#[tauri::command]
pub async fn get_sessions(
    state: State<'_, AppState>,
) -> Result<Vec<session::Session>, String> {
    eprintln!("🔍 get_sessions called");
    let state_guard = state.session_manager.lock().unwrap();
    if let Some(manager) = &*state_guard {
        let sessions = manager.get_sessions();
        eprintln!("🔍 Returning {} sessions", sessions.len());
        Ok(sessions)
    } else {
        eprintln!("❌ get_sessions: No pcap file loaded");
        Err("No pcap file loaded".to_string())
    }
}

#[tauri::command]
pub async fn get_message_detail(
    message_id: String,
    state: State<'_, AppState>,
) -> Result<Option<protocol::Message>, String> {
    let state_guard = state.session_manager.lock().unwrap();
    if let Some(manager) = &*state_guard {
        Ok(manager.get_message_by_id(&message_id).cloned())
    } else {
        Err("No pcap file loaded".to_string())
    }
}
