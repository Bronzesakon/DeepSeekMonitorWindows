<template>
  <div class="glass-card p-5">
    <span class="text-sm font-semibold text-gray-700 dark:text-gray-300 block mb-3">刷新间隔</span>
    <div class="grid grid-cols-4 gap-2">
      <button
        v-for="opt in options"
        :key="opt.value"
        @click="select(opt.value)"
        class="py-2 text-xs font-medium rounded-lg transition"
        :class="settings.refreshInterval === opt.value
          ? 'bg-brand-500 text-white shadow-sm shadow-brand-500/30'
          : 'bg-white/60 dark:bg-zinc-600/60 text-gray-800 dark:text-gray-200 border border-gray-400/40 dark:border-white/20 shadow-sm hover:bg-white/80 dark:hover:bg-zinc-500/70'"
      >
        {{ opt.label }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from "@/stores/settings";

const settings = useSettingsStore();

const options = [
  { value: 30, label: "30秒" },
  { value: 60, label: "60秒" },
  { value: 120, label: "2分钟" },
  { value: 300, label: "5分钟" },
];

function select(value: number) {
  settings.setRefreshInterval(value);
}
</script>
