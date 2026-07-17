<template>
  <div ref="rootEl" class="px-5 pt-2 pb-4 glass-root">
    <div data-tauri-drag-region class="title-bar mb-2">
      <ProviderToggle />
      <div class="title-actions">
        <button aria-label="关闭" class="title-btn close-btn" @click="closeWindow"><CloseIcon /></button>
      </div>
    </div>

    <div class="space-y-3">
      <div class="glass-card p-5">
        <div class="grid grid-cols-2 gap-3 items-center text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3"><span>{{ dateTitle }} Token 详情</span><span class="text-sm text-orange-500 dark:text-orange-400">¥{{ dayCost }}</span></div>
        <div class="grid grid-cols-2 gap-3 text-sm">
          <div>
            <div class="text-[11px] font-medium text-green-500 dark:text-green-500 mb-0.5">缓存命中</div>
            <div class="text-xl font-bold text-green-400 dark:text-green-400">{{ cacheHit.toLocaleString() }}</div>
          </div>
          <div>
            <div class="text-[11px] font-medium text-orange-500 dark:text-orange-500 mb-0.5">缓存未命中</div>
            <div class="text-xl font-bold text-orange-400 dark:text-orange-400">{{ cacheMiss.toLocaleString() }}</div>
          </div>
          <div>
            <div class="text-[11px] font-medium text-blue-500 dark:text-blue-500 mb-0.5">输出</div>
            <div class="text-xl font-bold text-blue-400 dark:text-blue-400">{{ output.toLocaleString() }}</div>
          </div>
          <div>
            <div class="text-[11px] font-medium text-purple-500 dark:text-purple-500 mb-0.5">命中率</div>
            <div class="text-xl font-bold text-purple-400 dark:text-purple-400">{{ cacheHitRate }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/dpi";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useThemeStore } from "@/stores/theme";
import ProviderToggle from "@/components/ProviderToggle.vue";
import CloseIcon from "@/components/icons/CloseIcon.vue";

const theme = useThemeStore();
theme.init();

const cacheHit = ref(0);
const cacheMiss = ref(0);
const output = ref(0);
const cacheHitRate = ref("--");
const dateTitle = ref("--");
const dayCost = ref("0.00");

const rootEl = ref<HTMLElement>();

function closeWindow() {
  getCurrentWindow().hide();
}

function applyData(payload: { date?: string; cacheHit: number; cacheMiss: number; output: number; cacheHitRate: string; cost?: number }) {
  cacheHit.value = payload.cacheHit;
  cacheMiss.value = payload.cacheMiss;
  output.value = payload.output;
  cacheHitRate.value = payload.cacheHitRate;
  dayCost.value = ((payload.cost ?? 0) / 100).toFixed(2);
  if (payload.date) {
    // Format "2026-06-13" → "06-13"
    const parts = payload.date.split("-");
    dateTitle.value = parts.length >= 3 ? `${parts[1]}-${parts[2]}` : payload.date;
  }
}

onMounted(async () => {
  // Pull initial data from shared state (avoids timing issues with emit)
  try {
    const initial = await invoke<{ date?: string; cacheHit: number; cacheMiss: number; output: number; cacheHitRate: string; cost?: number }>("get_trend_detail_data");
    if (initial && initial.cacheHitRate) {
      applyData(initial);
    }
  } catch (_) { /* ignore */ }

  // Listen for subsequent updates while window stays open
  listen<{ date?: string; cacheHit: number; cacheMiss: number; output: number; cacheHitRate: string; cost?: number }>(
    "trend-detail-data",
    (event) => { applyData(event.payload); }
  );

  await nextTick();
  if (rootEl.value) {
    const resize = () => {
      const h = rootEl.value!.scrollHeight;
      if (h > 0) getCurrentWindow().setSize(new LogicalSize(400, h));
    };
    resize();
    new ResizeObserver(() => resize()).observe(rootEl.value);
  }
});
</script>
