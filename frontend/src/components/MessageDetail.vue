<template>
  <div class="message-detail">
    <div v-if="message" class="detail-content">
      <div class="detail-header">
        <h3>Message Detail</h3>
        <div class="msg-title">
          <span v-if="message.method" class="method">{{ message.method }}</span>
          <span v-if="message.status_code" class="status">{{ message.status_code }} {{ message.status_text }}</span>
        </div>
      </div>

      <div class="detail-meta">
        <div class="meta-row">
          <span class="label">Time:</span>
          <span>{{ message.timestamp }}</span>
        </div>
        <div class="meta-row">
          <span class="label">From:</span>
          <span>{{ message.src_ip }}:{{ message.src_port }}</span>
        </div>
        <div class="meta-row">
          <span class="label">To:</span>
          <span>{{ message.dst_ip }}:{{ message.dst_port }}</span>
        </div>
        <div class="meta-row">
          <span class="label">Protocol:</span>
          <span>{{ message.protocol }}</span>
        </div>
      </div>

      <div class="tab-bar">
        <button
          v-for="tab in tabs"
          :key="tab"
          class="tab-btn"
          :class="{ active: activeTab === tab }"
          @click="activeTab = tab"
        >
          {{ tab }}
        </button>
      </div>

      <div class="tab-content">
        <div v-if="activeTab === 'Headers'" class="headers-tab">
          <table class="headers-table">
            <tr v-for="(value, key) in message.headers" :key="key">
              <td class="header-name">{{ key }}</td>
              <td class="header-value">{{ value }}</td>
            </tr>
          </table>
        </div>

        <div v-else-if="activeTab === 'Body'" class="body-tab">
          <pre v-if="message.body" class="body-content">{{ message.body }}</pre>
          <div v-else class="no-content">No message body</div>
        </div>

        <div v-else-if="activeTab === 'Raw'" class="raw-tab">
          <pre class="raw-content">{{ rawText }}</pre>
        </div>
      </div>
    </div>
    <div v-else class="empty-state">
      Select a message to view details
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { SipMessage } from '../types/sip';

const props = defineProps<{
  message: SipMessage | null;
}>();

const activeTab = ref('Headers');
const tabs = ['Headers', 'Body', 'Raw'];

const rawText = computed(() => {
  if (!props.message) return '';
  return new TextDecoder().decode(new Uint8Array(props.message.raw_data));
});
</script>

<style scoped>
.message-detail {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.detail-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.detail-header {
  padding: 12px 16px;
  background: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
}

.detail-header h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: #2c3e50;
}

.msg-title {
  font-size: 14px;
}

.method {
  padding: 2px 8px;
  background: #3498db;
  color: white;
  border-radius: 4px;
  font-weight: 600;
}

.status {
  padding: 2px 8px;
  background: #28a745;
  color: white;
  border-radius: 4px;
  font-weight: 600;
}

.detail-meta {
  padding: 12px 16px;
  background: white;
  border-bottom: 1px solid #eee;
}

.meta-row {
  display: flex;
  gap: 8px;
  margin-bottom: 4px;
  font-size: 13px;
}

.meta-row .label {
  color: #6c757d;
  min-width: 60px;
  font-weight: 500;
}

.tab-bar {
  display: flex;
  border-bottom: 1px solid #dee2e6;
  background: #f8f9fa;
}

.tab-btn {
  padding: 8px 16px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 13px;
  color: #495057;
  border-bottom: 2px solid transparent;
  transition: all 0.15s;
}

.tab-btn:hover {
  background: #e9ecef;
}

.tab-btn.active {
  color: #3498db;
  border-bottom-color: #3498db;
  font-weight: 600;
}

.tab-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.headers-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.headers-table tr {
  border-bottom: 1px solid #f0f0f0;
}

.headers-table td {
  padding: 6px 8px;
  vertical-align: top;
}

.header-name {
  font-weight: 600;
  color: #2c3e50;
  white-space: nowrap;
  min-width: 120px;
}

.header-value {
  color: #495057;
  word-break: break-all;
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 12px;
}

.body-content, .raw-content {
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 12px;
  white-space: pre-wrap;
  word-break: break-all;
  color: #2c3e50;
  margin: 0;
}

.no-content {
  color: #6c757d;
  font-style: italic;
  padding: 20px;
  text-align: center;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #6c757d;
  font-size: 14px;
}
</style>
