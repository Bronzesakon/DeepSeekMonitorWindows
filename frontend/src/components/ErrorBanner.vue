<template>
  <div v-if="error" class="bg-red-500/10 dark:bg-red-500/15 backdrop-blur-sm border border-red-500/20 dark:border-red-500/30 rounded-lg px-4 py-2.5 text-xs font-medium text-red-700 dark:text-red-300 text-center">
    <div v-for="(msg, i) in errorMessages" :key="i" :class="{ 'mt-1': i > 0 }">
      {{ msg }}
    </div>
    <button v-if="showSettings" @click="openSettings" class="mt-2 px-3 py-1 bg-red-500 hover:bg-red-600 text-white text-xs font-semibold rounded transition">
      去设置
    </button>
  </div>
  <div v-if="warning && !error" class="bg-amber-500/10 dark:bg-amber-500/15 backdrop-blur-sm border border-amber-500/20 dark:border-amber-500/30 rounded-lg px-4 py-2.5 text-xs font-medium text-amber-700 dark:text-amber-300 text-center">
    {{ warning }}
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = withDefaults(defineProps<{
  error: string | null;
  warning: string | null;
  showSettings?: boolean;
}>(), {
  showSettings: false
});

const errorMessages = computed(() => {
  if (!props.error) return [];
  return props.error.split('\n').filter(m => m.trim());
});

function openSettings() {
  invoke('show_settings_window');
}
</script>