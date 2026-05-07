<template>
  <div class="app-container">
    <Toolbar @open-file="handleOpenFile" :loading="loading" />
    <div class="main-content">
      <div class="left-panel">
        <div class="debug-info" v-if="sessions.length">
          {{ sessions.length }} sessions, {{ getTotalMessages() }} messages
        </div>
        <SessionList
          :sessions="sessions"
          :selected-id="selectedSessionKey"
          @select-session="handleSelectSession"
        />
      </div>
      <div class="center-panel">
        <MessageFlow
          :messages="selectedMessages"
          :selected-id="selectedMessageId"
          @select-message="handleSelectMessage"
        />
      </div>
      <div class="right-panel">
        <MessageDetail :message="selectedMessage" />
      </div>
    </div>
    <div v-if="error" class="error-bar">
      {{ error }}
      <button @click="error = ''">×</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import Toolbar from './components/Toolbar.vue';
import SessionList from './components/SessionList.vue';
import MessageFlow from './components/MessageFlow.vue';
import MessageDetail from './components/MessageDetail.vue';
import type { Session, SipMessage } from './types/sip';

const sessions = ref<Session[]>([]);
const selectedSessionKey = ref<string>('');
const selectedMessages = ref<SipMessage[]>([]);
const selectedMessage = ref<SipMessage | null>(null);
const selectedMessageId = ref<string>('');
const loading = ref(false);
const error = ref('');

async function handleOpenFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'PCAP Files',
        extensions: ['pcap', 'pcapng', 'cap']
      }]
    });

    if (selected) {
      loading.value = true;
      error.value = '';

      // Open file and parse
      console.log('Opening file:', selected);
      const messages = await invoke<SipMessage[]>('open_pcap_file', { path: selected as string });
      console.log('Messages received:', messages?.length || 0);

      // Get sessions
      sessions.value = await invoke<Session[]>('get_sessions');
      console.log('Sessions loaded:', sessions.value?.length || 0);

      // Reset selection
      selectedSessionKey.value = '';
      selectedMessages.value = [];
      selectedMessage.value = null;
      selectedMessageId.value = '';

      if (sessions.value.length === 0) {
        error.value = 'No SIP sessions found in file';
      }
    }
  } catch (e: any) {
    console.error('Error opening file:', e);
    error.value = `Error: ${e}`;
  } finally {
    loading.value = false;
  }
}

function handleSelectSession(session: Session) {
  // Use call_id as key (grouped by Call-ID only)
  selectedSessionKey.value = session.dialog_key.call_id;
  selectedMessages.value = session.messages;
  selectedMessage.value = null;
  selectedMessageId.value = '';
}

function handleSelectMessage(message: SipMessage) {
  selectedMessage.value = message;
  selectedMessageId.value = message.id;
}

function getTotalMessages(): number {
  return sessions.value.reduce((sum, s) => sum + s.messages.length, 0);
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.left-panel {
  width: 300px;
  min-width: 250px;
  border-right: 1px solid #ddd;
  overflow-y: auto;
  background: white;
}

.center-panel {
  flex: 1;
  overflow-y: auto;
  background: white;
}

.right-panel {
  width: 450px;
  min-width: 350px;
  border-left: 1px solid #ddd;
  overflow-y: auto;
  background: white;
}

.debug-info {
  padding: 8px 16px;
  background: #e3f2fd;
  border-bottom: 1px solid #ddd;
  font-size: 12px;
  color: #1976d2;
}

.error-bar {
  padding: 10px 20px;
  background: #ff5252;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.error-bar button {
  background: none;
  border: none;
  color: white;
  font-size: 20px;
  cursor: pointer;
}
</style>
