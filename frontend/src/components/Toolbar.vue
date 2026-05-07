<template>
  <div class="toolbar">
    <button @click="$emit('open-file')" :disabled="loading" class="btn-open">
      📂 Open PCAP File
    </button>
    <span v-if="loading" class="loading-text">Loading...</span>
    <div class="toolbar-right">
      <span class="session-count" v-if="sessionCount !== null">
        {{ sessionCount }} sessions
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  loading: boolean;
  sessionCount?: number | null;
}>();

defineEmits<{
  'open-file': [];
}>();
</script>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  padding: 10px 16px;
  background: #2c3e50;
  color: white;
  gap: 12px;
}

.btn-open {
  padding: 8px 16px;
  background: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.btn-open:hover {
  background: #2980b9;
}

.btn-open:disabled {
  background: #7f8c8d;
  cursor: not-allowed;
}

.loading-text {
  color: #ecf0f1;
  font-size: 14px;
}

.toolbar-right {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: 12px;
}

.session-count {
  font-size: 13px;
  opacity: 0.8;
}
</style>
