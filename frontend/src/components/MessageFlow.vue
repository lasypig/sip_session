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
          <span class="arrow">→</span>
          <span class="ip">{{ msg.dst_ip }}:{{ msg.dst_port }}</span>
        </div>
        <div class="msg-type">
          <span v-if="msg.method" class="method-badge">{{ msg.method }}</span>
          <span v-if="msg.status_code" class="status-badge" :class="statusClass(msg.status_code)">
            {{ msg.status_code }} {{ msg.status_text }}
          </span>
        </div>
        <div class="msg-protocol">
          <small>{{ msg.protocol }}</small>
        </div>
      </div>
      <div v-if="messages.length === 0" class="empty-state">
        Select a session to view messages
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SipMessage } from '../types/sip';

defineProps<{
  messages: SipMessage[];
  selectedId?: string;
}>();

defineEmits<{
  'select-message': [message: SipMessage];
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

function statusClass(code: number): string {
  if (code >= 100 && code < 200) return 'provisional';
  if (code >= 200 && code < 300) return 'success';
  if (code >= 300 && code < 400) return 'redirect';
  if (code >= 400 && code < 500) return 'client-error';
  if (code >= 500) return 'server-error';
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
  font-size: 13px;
  color: #6c757d;
}

.flow-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.message-item {
  padding: 10px 12px;
  margin-bottom: 6px;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
}

.message-item:hover {
  border-color: #3498db;
  background: #f8f9fa;
}

.message-item.active {
  border-color: #2196f3;
  background: #e3f2fd;
}

.msg-timestamp {
  font-size: 11px;
  color: #6c757d;
  font-family: 'Monaco', 'Consolas', monospace;
  margin-bottom: 4px;
}

.msg-direction {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
  font-size: 13px;
}

.msg-direction .ip {
  font-family: 'Monaco', 'Consolas', monospace;
  color: #495057;
}

.msg-direction .arrow {
  color: #3498db;
  font-weight: bold;
}

.msg-type {
  margin-bottom: 4px;
}

.method-badge {
  padding: 2px 8px;
  background: #3498db;
  color: white;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}

.status-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}

.status-badge.provisional {
  background: #fff3cd;
  color: #856404;
}

.status-badge.success {
  background: #d4edda;
  color: #155724;
}

.status-badge.redirect {
  background: #cce5ff;
  color: #004085;
}

.status-badge.client-error {
  background: #f8d7da;
  color: #721c24;
}

.status-badge.server-error {
  background: #f5c6cb;
  color: #721c24;
}

.msg-protocol {
  font-size: 10px;
  color: #868e96;
}

.empty-state {
  padding: 40px 20px;
  text-align: center;
  color: #6c757d;
  font-size: 14px;
}
</style>
