export interface SipMessage {
  id: string;
  timestamp: string;
  src_ip: string;
  dst_ip: string;
  src_port: number;
  dst_port: number;
  protocol?: string;
  method?: string;
  status_code?: number;
  status_text?: string;
  headers: Record<string, string>;
  body?: string;
  raw_data: number[];
}

export interface DialogKey {
  call_id: string;
  from_tag: string;
  to_tag: string;
}

export interface Session {
  dialog_key: DialogKey;
  messages: SipMessage[];
  start_time: string;
  end_time?: string;
  state: string;
}
