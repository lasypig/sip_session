<template>
  <div class="session-list">
    <div class="list-header">
      <h3>SIP/RTSP Sessions</h3>
      <span class="count">{{ sessions.length }}</span>
    </div>
    <div class="list-content">
      <div
        v-for="session in sessions"
        :key="sessionKey(session)"
        class="session-item"
        :class="{ active: selectedId === sessionKey(session) }"
        @click="$emit('select-session', session)"
      >
        <div class="session-header">
          <span class="protocol-badge" :class="session.key.type.toLowerCase()">
            {{ session.key.type }}
          </span>
          <span class="state-badge" :class="session.state.toLowerCase()">
            {{ session.state }}
          </span>
        </div>
        <div class="session-identifier" :title="getSessionTitle(session)">
          {{ getSessionDisplay(session) }}
        </div>
        <div class="session-meta">
          <span>{{ formatTime(session.start_time) }}</span>
          <span>{{ session.messages.length }} msgs</span>
        </div>
      </div>
      <div v-if="sessions.length === 0" class="empty-state">
        No sessions yet. Open a pcap file.
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Session } from '../types/sip';

const props = defineProps<{
  sessions: Session[];
  selectedId?: string;
}>();

defineEmits<{
  'select-session': [session: Session];
}>();

function sessionKey(session: Session): string {
  if (session.key.type === 'SIP') {
    return session.key.call_id;
  } else {
    return session.key.session_id;
  }
}

function getSessionTitle(session: Session): string {
  if (session.key.type === 'SIP') {
    return `SIP Call-ID: ${session.key.call_id}`;
  } else {
    return `RTSP: ${session.key.session_id}`;
  }
}

function getSessionDisplay(session: Session): string {
  if (session.key.type === 'SIP') {
    return truncate(session.key.call_id, 28);
  } else {
    return `${session.key.session_id}`;
  }
}

function truncate(str: string, len: number): string {
  return str.length > len ? str.substring(0, len) + '...' : str;
}

function formatTime(timestamp: string): string {
  if (!timestamp) return '';
  const parts = timestamp.split('.');
  if (parts.length >= 1) {
    return parts[0];
  }
  return timestamp;
}
</script>

<style scoped>
.session-list {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
}

.list-header h3 {
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

.list-content {
  flex: 1;
  overflow-y: auto;
  height: calc(100% - 48px); /* Subtract header height */
}

.session-item {
  padding: 12px 16px;
  border-bottom: 1px solid #eee;
  cursor: pointer;
  transition: background 0.15s;
}

.session-item:hover {
  background: #f8f9fa;
}

.session-item.active {
  background: #e3f2fd;
  border-left: 3px solid #2196f3;
}

.session-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.protocol-badge {
  padding: 2px 6px;
  border-radius: 8px;
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
}

.protocol-badge.sip {
  background: #e8f5e9;
  color: #2e7d32;
}

.protocol-badge.rtsp {
  background: #fff3e0;
  color: #e65100;
}

.state-badge {
  padding: 2px 6px;
  border-radius: 8px;
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
}

.state-badge.early,
.state-badge.idle {
  background: #fff3cd;
  color: #856404;
}

.state-badge.confirmed,
.state-badge.ready,
.state-badge.playing {
  background: #d4edda;
  color: #155724;
}

.state-badge.terminated {
  background: #f8d7da;
  color: #721c24;
}

.state-badge.paused {
  background: #cce5ff;
  color: #004085;
}

.session-identifier {
  font-size: 13px;
  color: #495057;
  font-family: 'Monaco', 'Consolas', monospace;
  margin-bottom: 4px;
  word-break: break-all;
}

.session-meta {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: #6c757d;
  margin-bottom: 4px;
}

.empty-state {
  padding: 40px 20px;
  text-align: center;
  color: #6c757d;
  font-size: 14px;
}
</style>
