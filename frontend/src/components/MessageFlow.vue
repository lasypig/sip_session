<template>
  <div class="message-flow">
    <div class="flow-header">
      <h3>Message Flow</h3>
      <span class="count" v-if="messages.length">{{ messages.length }} messages</span>
    </div>
    <div class="flow-content">
      <div
        v-for="msg in messages"
        :key="msg.id"
        class="message-item"
        :class="{ active: selectedId === msg.id }"
        @click="$emit('select-message', msg)"
      >
        <div class="msg-timestamp">{{ formatTimestamp(msg.timestamp) }}</div>
        <div class="msg-direction">
          <span class="ip">{{ msg.src_ip }}:{{ msg.src_port }}</span>
          <span class="arrow">{{ getProtocolArrow(msg.protocol) }}</span>
          <span class="ip">{{ msg.dst_ip }}:{{ msg.dst_port }}</span>
        </div>
        <div class="msg-type">
          <span v-if="msg.method" class="method-badge" :class="getProtocolClass(msg.protocol)">
            {{ msg.method }}
          </span>
          <span v-if="msg.status_code" class="status-badge" :class="statusClass(msg.status_code)">
            {{ msg.status_code }} {{ msg.status_text }}
          </span>
        </div>
        <div class="msg-protocol">
          <small :class="protocolColor(msg.protocol)">{{ msg.protocol }}</small>
        </div>
      </div>
      <div v-if="messages.length === 0" class="empty-state">
        Select a session to view messages
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Message } from '../types/sip';

defineProps<{
  messages: Message[];
  selectedId?: string;
}>();

defineEmits<{
  'select-message': [message: Message];
}>();

function formatTimestamp(ts: string): string {
  const parts = ts.split('.');
  if (parts.length === 2) {
    // Show seconds.microseconds
    const secs = parts[0];
    const micros = parts[1].padEnd(6, '0').substring(0, 6);
    return `${secs}.${micros}`;
  }
  return ts;
}

function getProtocolArrow(protocol: string): string {
  switch (protocol) {
    case 'RTSP':
      return '⏯️';
    case 'SIP':
    default:
      return '→';
  }
}

function getProtocolClass(protocol: string): string {
  switch (protocol) {
    case 'RTSP':
      return 'rtsp-method';
    case 'SIP':
    default:
      return 'sip-method';
  }
}

function protocolColor(protocol: string): string {
  switch (protocol) {
    case 'RTSP':
      return 'rtsp-text';
    case 'SIP':
    default:
      return 'sip-text';
  }
}

function statusClass(status: number): string {
  if (status >= 200 && status < 300) {
    return 'status-success';
  } else if (status >= 300 && status < 400) {
    return 'status-redirect';
  } else if (status >= 400 && status < 500) {
    return 'status-client-error';
  } else if (status >= 500) {
    return 'status-server-error';
  }
  return '';
}
</script>

<style scoped>
.message-flow {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.flow-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
}

.flow-header h3 {
  margin: 0;
  font-size: 16px;
  color: #2c3e50;
}

.count {
  background: #3498db;
  color: white;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.flow-content {
  flex: 1;
  overflow-y: auto;
}

.message-item {
  padding: 12px 16px;
  border-bottom: 1px solid #eee;
  cursor: pointer;
  transition: background 0.15s;
}

.message-item:hover {
  background: #f8f9fa;
}

.message-item.active {
  background: #e3f2fd;
  border-left: 3px solid #2196f3;
}

.msg-timestamp {
  font-size: 11px;
  color: #6c757d;
  margin-bottom: 4px;
  font-family: 'Monaco', 'Consolas', monospace;
}

.msg-direction {
  display: flex;
  align-items: center;
  margin-bottom: 4px;
  font-size: 12px;
}

.ip {
  color: #495057;
  font-family: 'Monaco', 'Consolas', monospace;
}

.arrow {
  margin: 0 8px;
  color: #6c757d;
}

.msg-type {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.method-badge {
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
}

.sip-method {
  background: #e3f2fd;
  color: #1976d2;
}

.rtsp-method {
  background: #fff3e0;
  color: #f57c00;
}

.status-badge {
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
}

.status-success {
  background: #d4edda;
  color: #155724;
}

.status-redirect {
  background: #fff3cd;
  color: #856404;
}

.status-client-error {
  background: #f8d7da;
  color: #721c24;
}

.status-server-error {
  background: #f5c6cb;
  color: #721c24;
}

.msg-protocol {
  font-size: 10px;
}

.sip-text {
  color: #1976d2;
}

.rtsp-text {
  color: #f57c00;
}

.empty-state {
  padding: 40px 20px;
  text-align: center;
  color: #6c757d;
  font-size: 14px;
}
</style>
