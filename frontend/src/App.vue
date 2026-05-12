<template>
  <div class="app-container"
       :class="{ 'drag-active': isDragging }">
    <Toolbar @open-file="handleOpenFile" :loading="loading" />
    <div class="main-content"
         @mousemove="handleMouseMove"
         @mouseup="handleMouseUp"
         :class="{ 'resizing': isResizing }">
      <div class="left-panel" :style="{ width: leftPanelWidth + 'px' }">
        <SessionList
          :sessions="sessions"
          :selected-id="selectedSessionKey"
          @select-session="handleSelectSession"
        />
        <div class="debug-info-bottom" v-if="sessions.length">
          {{ sessions.length }} sessions, {{ getTotalMessages() }} messages
        </div>
      </div>
      <div class="resizer left-resizer"
           @mousedown="startResize('left')"
           :class="{ 'resizing': isResizing }"></div>
      <div class="center-panel" :style="{ width: centerPanelWidth + 'px' }">
        <MessageFlow
          :messages="selectedMessages"
          :selected-id="selectedMessageId"
          @select-message="handleSelectMessage"
        />
      </div>
      <div class="resizer right-resizer"
           @mousedown="startResize('right')"
           :class="{ 'resizing': isResizing }"></div>
	  <div class="right-panel" :style="{ width: rightPanelWidth + 'px', flexShrink: 0 }">
        <MessageDetail :message="selectedMessage" />
      </div>
    </div>
    <div v-if="error" class="error-bar">
      {{ error }}
      <button @click="error = ''">×</button>
    </div>
    <!-- Drag overlay -->
    <div v-if="isDragging" class="drag-overlay">
      <div class="drag-content">
        <p class="drag-icon">📁</p>
        <p class="drag-text">Drop PCAP file here</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import type { Event } from '@tauri-apps/api/event'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import type { DragDropEvent } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import Toolbar from './components/Toolbar.vue';
import SessionList from './components/SessionList.vue';
import MessageFlow from './components/MessageFlow.vue';
import MessageDetail from './components/MessageDetail.vue';
import type { Session, Message } from './types/sip';

const sessions = ref<Session[]>([]);
const selectedSessionKey = ref<string>('');
const selectedMessages = ref<Message[]>([]);
const selectedMessage = ref<Message | null>(null);
const selectedMessageId = ref<string>('');
const loading = ref(false);
const error = ref('');

const isDragging = ref(false);
const isResizing = ref(false);
const currentResizer = ref<'left' | 'right' | null>(null);
const leftPanelWidth = ref<number>(300);
const centerPanelWidth = ref<number>(300);
const rightPanelWidth = ref<number>(800);
const startX = ref<number>(0);
const startWidths = ref<{ left: number, center: number, right: number } | null>(null);

let unlisten: (() => void) | null = null;

async function handleFileDropped(file: string) {
  // Open file and parse
	  loading.value = true;
  console.log('Opening file:', file);
  const messages = await invoke<Message[]>('open_pcap_file', { path: file as string });
  console.log('Messages received:', messages?.length || 0);

  // Get sessions
  try {
    sessions.value = await invoke<Session[]>('get_sessions');
    console.log('Sessions loaded:', sessions.value?.length || 0);
    console.log('Session data sample:', sessions.value.slice(0, 2));
    console.log('First session type:', sessions.value[0]?.key.type);
    console.log('First session messages count:', sessions.value[0]?.messages.length);
  } catch (e) {
    console.error('Error getting sessions:', e);
  }

  // Reset selection
  selectedSessionKey.value = '';
  selectedMessages.value = [];
  selectedMessage.value = null;
  selectedMessageId.value = '';

  if (sessions.value.length === 0) {
	error.value = 'No SIP sessions found in file';
  }
  loading.value = false;
}

onMounted( async () => {
  try {
    const webview = getCurrentWebview()
    
    unlisten = await webview.onDragDropEvent((event: Event<DragDropEvent>) => {
      console.log('📥 DragDropEvent:', event.payload.type, event.payload) // 强烈建议打印

      if (event.payload.type === 'enter') {
        isDragging.value = true
      } else if (event.payload.type === 'drop') {
        isDragging.value = false
        const file = event.payload.paths || []
        
		if (file.length === 0) {
		  console.log('No files in drop event');
		  return;
		}
        
		const validExtensions = ['pcap', 'pcapng', 'cap'];
		const fileExtension = file[0].split('.').pop()?.toLowerCase();

		if (!validExtensions.includes(fileExtension || '')) {
		  error.value = `Invalid file type. Please drop a PCAP file (${validExtensions.join(', ')})`;
		  return;
		} else {
			console.log('✅ Valid file type detected:', file);
		}

		// Reuse the processFile logic from handleOpenFile
		error.value = '';

		handleFileDropped(file[0]);
	  } else if (['leave', 'cancel'].includes(event.payload.type)) {
		isDragging.value = false
	  }
	})

	console.log('✅ DragDropEvent 监听器已成功注册')
  } catch (err) {
	console.error('❌ 注册拖拽监听失败：', err)
  }
})

onUnmounted(() => {
  unlisten?.()
  document.removeEventListener('mousemove', handleMouseMove as any);
  document.removeEventListener('mouseup', handleMouseUp);
})

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
	  error.value = '';
	  await handleFileDropped(selected);
    }
  } catch (e: any) {
    console.error('Error opening file:', e);
    error.value = `Error: ${e}`;
  } finally {
    loading.value = false;
  }
}

function handleSelectSession(session: Session) {
  // Use session key based on protocol
  selectedSessionKey.value = session.key.type === 'SIP'
    ? session.key.call_id
    : session.key.session_id;
  selectedMessages.value = session.messages;
  selectedMessage.value = null;
  selectedMessageId.value = '';
}

function handleSelectMessage(message: Message) {
  selectedMessage.value = message;
  selectedMessageId.value = message.id;
}

function getTotalMessages(): number {
  const total = sessions.value.reduce((sum, s) => sum + s.messages.length, 0);
  console.log('Total messages calculated:', total);
  return total;
}

// Watch for changes
watch(sessions, (newSessions, oldSessions) => {
  console.log('Sessions changed:', newSessions?.length || 0, 'old:', oldSessions?.length || 0);
  console.log('New sessions data:', newSessions);
}, { deep: true });

function startResize(resizer: 'left' | 'right') {
  isResizing.value = true;
  currentResizer.value = resizer;
  startX.value = window.event?.clientX || 0;

  const mainContent = document.querySelector('.main-content');
  if (mainContent) {
    const rect = mainContent.getBoundingClientRect();
    startWidths.value = {
      left: leftPanelWidth.value,
      center: centerPanelWidth.value,
      right: rightPanelWidth.value
    };
  }
}

function handleMouseMove(event: MouseEvent) {
  if (!isResizing.value || !currentResizer.value || !startWidths.value) return;

  const deltaX = event.clientX - startX.value;

  if (currentResizer.value === 'left') {
    const newLeft = Math.max(250, Math.min(600, startWidths.value.left + deltaX));
    const newCenter = startWidths.value.center - deltaX;

    leftPanelWidth.value = newLeft;
    centerPanelWidth.value = Math.max(200, newCenter);
  } else if (currentResizer.value === 'right') {
    const newRight = Math.max(400, Math.min(800, startWidths.value.right - deltaX));
    const newCenter = startWidths.value.center + deltaX;

    rightPanelWidth.value = newRight;
    centerPanelWidth.value = Math.max(200, newCenter);
  }
}

function handleMouseUp() {
  isResizing.value = false;
  currentResizer.value = null;
  startWidths.value = null;
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
  min-width: 250px;
  max-width: 600px;
  border-right: 1px solid #ddd;
  overflow-y: auto;
  background: white;
}

.center-panel {
  min-width: 250px;
  overflow-y: auto;
  background: white;
}

.right-panel {
  min-width: 500px;
  max-width: 800px;
  border-left: 1px solid #ddd;
  overflow-y: auto;
  background: white;
  margin-right: 0;
  padding-right: 0;
}

.resizer {
  width: 3px;
  background: transparent;
  cursor: col-resize;
  transition: background-color 0.2s;
}

.resizer:hover {
  background: #3498db;
}

.resizer.left-resizer {
  border-right: 1px solid #ddd;
}

.resizer.right-resizer {
  border-left: 1px solid #ddd;
  margin-right: 0;
}

.resizer.resizing {
  background: #3498db;
  background: linear-gradient(to right, #3498db, transparent);
}

.main-content.resizing {
  cursor: col-resize;
}

.debug-info-bottom {
  padding: 8px 16px;
  background: #e3f2fd;
  border-bottom: 1px solid #ddd;
  font-size: 12px;
  color: #1976d2;
  position: sticky;
  bottom: 0;
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

.app-container.drag-active {
  outline: 3px dashed #3498db;
  outline-offset: -3px;
}

.drag-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(52, 152, 219, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
  z-index: 1000;
}

.drag-content {
  text-align: center;
  padding: 40px;
  background: rgba(255, 255, 255, 0.95);
  border: 2px dashed #3498db;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.drag-icon {
  font-size: 48px;
  margin: 0 0 12px 0;
  line-height: 1;
}

.drag-text {
  color: #3498db;
  font-size: 18px;
  font-weight: 500;
  margin: 0;
}
</style>
