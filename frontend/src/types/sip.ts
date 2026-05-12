export interface Message {
  id: string;
  timestamp: string;
  src_ip: string;
  dst_ip: string;
  src_port: number;
  dst_port: number;
  protocol: 'SIP' | 'RTSP';
  method?: string;
  status_code?: number;
  status_text?: string;
  headers: Record<string, string>;
  body?: string;
  raw_data: number[];
}

export type SessionKey =
  | { type: 'SIP'; call_id: string }
  | { type: 'RTSP'; src_ip: string; src_port: number; dst_ip: string; dst_port: number };

export interface Session {
  key: SessionKey;
  messages: Message[];
  start_time: string;
  end_time?: string;
  state: string;
}