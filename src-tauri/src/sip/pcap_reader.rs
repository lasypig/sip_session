use pcap_file::pcap::{PcapReader, PcapPacket};
use pcap_file::pcapng::{PcapNgReader, Block};
use etherparse::{PacketHeaders, NetHeaders, TransportHeader};
use std::io::Cursor;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedPacket {
    pub timestamp: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub payload: Vec<u8>,
}

pub fn read_pcap_file(path: &Path) -> Result<Vec<ParsedPacket>, String> {
    eprintln!("Reading file: {:?}", path);
    let data = std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    eprintln!("File size: {} bytes", data.len());

    // Try as pcap format first
    let mut cursor = Cursor::new(data);
    if let Ok(mut pcap_reader) = PcapReader::new(&mut cursor) {
        eprintln!("Parsing as pcap format");
        return parse_pcap_packets(&mut pcap_reader);
    }

    // If pcap fails, try pcapng format
    let data = std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let cursor = Cursor::new(data);
    if let Ok(mut pcapng_reader) = PcapNgReader::new(cursor) {
        eprintln!("Parsing as pcapng format");
        return parse_pcapng_packets(&mut pcapng_reader);
    }

    Err("Failed to parse file as pcap or pcapng".to_string())
}

fn parse_pcap_packets<R: std::io::Read + std::io::Seek>(reader: &mut PcapReader<R>) -> Result<Vec<ParsedPacket>, String> {
    let mut packets = Vec::new();

    while let Some(result) = reader.next_packet() {
        match result {
            Ok(packet) => {
                if let Some(parsed) = extract_sip_payload(&packet) {
                    packets.push(parsed);
                }
            }
            Err(e) => eprintln!("Error reading packet: {}", e),
        }
    }

    Ok(packets)
}

fn parse_pcapng_packets<R: std::io::Read + std::io::Seek>(reader: &mut PcapNgReader<R>) -> Result<Vec<ParsedPacket>, String> {
    let mut packets = Vec::new();

    while let Some(result) = reader.next_block() {
        match result {
            Ok(block) => {
                if let Block::Packet(packet_block) = block {
                    let timestamp_ns = packet_block.timestamp;
                    let data = &packet_block.data;
                    if let Some(parsed) = extract_sip_from_data(data, timestamp_ns) {
                        packets.push(parsed);
                    }
                }
            }
            Err(e) => eprintln!("Error reading block: {}", e),
        }
    }

    Ok(packets)
}

fn format_ipv4(addr: &[u8; 4]) -> String {
    format!("{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3])
}

fn extract_sip_from_data(data: &[u8], timestamp_ns: u64) -> Option<ParsedPacket> {
    // Debug: Print first bytes
    if !data.is_empty() {
        let preview_len = std::cmp::min(36, data.len());
        eprintln!("Packet data (first {} bytes): {:02x?}", preview_len, &data[..preview_len]);
    }

    let mut headers = None;
    let mut eth_net_none = false;

    // Try 1: Parse as Ethernet frame
    match PacketHeaders::from_ethernet_slice(data) {
        Ok(h) => {
            eprintln!("Parsed as Ethernet frame, net={:?}", h.net.is_some());
            if h.net.is_some() {
                headers = Some(h);
            } else {
                // Ethernet parsed OK but no IP header - might be SLL in disguise
                eprintln!("Ethernet parsed but no IP header, will try SLL");
                eth_net_none = true;
            }
        }
        Err(e) => {
            eprintln!("Ethernet parse failed: {:?}, trying other formats", e);
        }
    }

    // Try 2: If Ethernet failed or parsed but no IP header, try SLL
    if headers.is_none() && (eth_net_none || data.len() > 16) {
        // SLL header: 2(type) + 2(ARPHRD) + 2(addr len) + 8(addr) + 2(protocol)
        // Try both byte orders for protocol type
        let proto_be = u16::from_be_bytes([data[14], data[15]]);
        let proto_le = u16::from_le_bytes([data[14], data[15]]);
        eprintln!("SLL protocol type BE=0x{:04x} LE=0x{:04x}", proto_be, proto_le);

        // 0x0800 = IPv4, 0x86DD = IPv6
        if proto_be == 0x0800 || proto_be == 0x86DD || proto_le == 0x0800 || proto_le == 0x86DD {
            eprintln!("Detected SLL header, skipping 16 bytes, trying IP parse");
            // Skip 16-byte SLL header and parse as IP
            match PacketHeaders::from_ip_slice(&data[16..]) {
                Ok(h) => {
                    eprintln!("Parsed as IP after SLL skip, net={:?}", h.net.is_some());
                    if h.net.is_some() {
                        headers = Some(h);
                    } else {
                        eprintln!("WARNING: from_ip_slice succeeded but net is None!");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse IP after SLL skip: {:?}", e);
                }
            }
        }
    }

    // Try 3: If still failed, try raw IP directly
    if headers.is_none() {
        eprintln!("Trying raw IP parse");
        match PacketHeaders::from_ip_slice(data) {
            Ok(h) => {
                eprintln!("Parsed as raw IP, net={:?}", h.net.is_some());
                if h.net.is_some() {
                    headers = Some(h);
                } else {
                    eprintln!("WARNING: from_ip_slice (raw) succeeded but net is None!");
                }
            }
            Err(e) => {
                eprintln!("Failed to parse as raw IP: {:?}", e);
            }
        }
    }

    let headers = match headers {
        Some(h) => h,
        None => {
            eprintln!("All parse attempts failed");
            return None;
        }
    };

    // Get IP addresses from net headers
    let (src_ip, dst_ip) = match &headers.net {
        Some(NetHeaders::Ipv4(ipv4_header, _)) => {
            (format_ipv4(&ipv4_header.source), format_ipv4(&ipv4_header.destination))
        }
        Some(NetHeaders::Ipv6(ipv6_header, _)) => {
            (ipv6_header.source_addr().to_string(), ipv6_header.destination_addr().to_string())
        }
        None => {
            eprintln!("No IP header found in packet");
            return None;
        }
    };

    // Get transport layer info and payload
    let (src_port, dst_port, protocol, payload_vec) = match &headers.transport {
        Some(TransportHeader::Udp(udp)) => {
            let payload = headers.payload.slice().to_vec();
            (udp.source_port, udp.destination_port, "UDP".to_string(), payload)
        }
        Some(TransportHeader::Tcp(tcp)) => {
            let payload = headers.payload.slice().to_vec();
            if payload.is_empty() {
                return None;
            }
            (tcp.source_port, tcp.destination_port, "TCP".to_string(), payload)
        }
        _ => {
            eprintln!("No UDP or TCP header found");
            return None;
        }
    };

    // Check if it's SIP traffic
    let is_sip_port = src_port == 5060 || src_port == 5061 ||
                     dst_port == 5060 || dst_port == 5061;

    if !is_sip_port && !is_sip_payload(&payload_vec) {
        return None;
    }

    // Format timestamp from nanoseconds
    let timestamp_str = format_timestamp_ns(timestamp_ns);

    Some(ParsedPacket {
        timestamp: timestamp_str,
        src_ip,
        dst_ip,
        src_port,
        dst_port,
        protocol,
        payload: payload_vec,
    })
}

fn extract_sip_payload(packet: &PcapPacket) -> Option<ParsedPacket> {
    // Convert Duration to nanoseconds
    let timestamp_ns = packet.timestamp.as_secs() * 1_000_000_000 + packet.timestamp.subsec_nanos() as u64;
    let result = extract_sip_from_data(packet.data.as_ref(), timestamp_ns);
    if result.is_none() {
        // Debug: print first 100 bytes of non-SIP packet
        let preview = &packet.data.as_ref()[..std::cmp::min(100, packet.data.len())];
        if let Ok(text) = String::from_utf8(preview.to_vec()) {
            eprintln!("Non-SIP packet (first 100 chars): {}", text.lines().next().unwrap_or(""));
        }
    }
    result
}

fn format_timestamp_ns(timestamp_ns: u64) -> String {
    let secs = timestamp_ns / 1_000_000_000;
    let nanos = timestamp_ns % 1_000_000_000;
    let micros = nanos / 1000;
    format!("{}.{:06}", secs, micros)
}

fn is_sip_payload(payload: &[u8]) -> bool {
    if let Ok(text) = String::from_utf8(payload[..std::cmp::min(20, payload.len())].to_vec()) {
        let upper = text.to_uppercase();
        upper.starts_with("INVITE ") ||
        upper.starts_with("ACK ") ||
        upper.starts_with("BYE ") ||
        upper.starts_with("CANCEL ") ||
        upper.starts_with("REGISTER ") ||
        upper.starts_with("OPTIONS ") ||
        upper.starts_with("SIP/2.0")
    } else {
        false
    }
}
