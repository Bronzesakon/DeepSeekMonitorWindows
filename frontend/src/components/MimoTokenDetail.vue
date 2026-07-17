<template>
  <div class="glass-card p-5">
    <div class="flex items-center justify-between mb-3">
      <span class="text-sm font-semibold text-gray-700 dark:text-gray-300">今日 Token 详情</span>
      <div class="flex gap-1">
        <button
          v-for="m in tabs"
          :key="m.value"
          @click="activeTab = m.value"
          class="px-2 py-0.5 rounded-full text-[11px] font-medium transition"
          :class="activeTab === m.value
            ? 'bg-brand-500 text-white shadow-sm shadow-brand-500/20'
            : 'bg-white/80 dark:bg-zinc-600/70 text-gray-800 dark:text-gray-200 border border-gray-400/40 dark:border-white/20 shadow-sm'"
        >
          {{ m.label }}
        </button>
      </div>
    </div>

    <!-- Standard model detail: cache hit/miss/output/rate -->
    <div v-if="activeTab !== 'asr'" class="grid grid-cols-2 gap-3 text-sm">
      <div>
        <div class="text-xs font-medium text-green-500 dark:text-green-500 mb-0.5">缓存命中</div>
        <div class="text-xl font-bold text-green-400 dark:text-green-400">{{ detailData.cacheHit.toLocaleString() }}</div>
      </div>
      <div>
        <div class="text-xs font-medium text-orange-500 dark:text-orange-500 mb-0.5">缓存未命中</div>
        <div class="text-xl font-bold text-orange-400 dark:text-orange-400">{{ detailData.cacheMiss.toLocaleString() }}</div>
      </div>
      <div>
        <div class="text-xs font-medium text-blue-500 dark:text-blue-500 mb-0.5">输出</div>
        <div class="text-xl font-bold text-blue-400 dark:text-blue-400">{{ detailData.output.toLocaleString() }}</div>
      </div>
      <div>
        <div class="text-xs font-medium text-purple-500 dark:text-purple-500 mb-0.5">命中率</div>
        <div class="text-xl font-bold text-purple-400 dark:text-purple-400">{{ detailData.hitRate }}</div>
      </div>
      <div v-if="store.currentDayAudioDuration > 0" class="col-span-2">
        <div class="text-xs font-medium text-cyan-500 dark:text-cyan-500 mb-0.5">音频转写时长</div>
        <div class="text-xl font-bold text-cyan-400 dark:text-cyan-400">{{ formatDuration(store.currentDayAudioDuration) }}</div>
      </div>
    </div>

    <!-- ASR detail: audio duration only (ASR is a dedicated model, not V2.5/Pro) -->
    <div v-else class="text-sm">
      <div v-if="store.currentDayAudioDuration > 0">
        <div class="text-xs font-medium text-cyan-500 dark:text-cyan-500 mb-0.5">当日已转写时长</div>
        <div class="text-2xl font-bold text-cyan-400 dark:text-cyan-400">{{ formatDuration(store.currentDayAudioDuration) }}</div>
      </div>
      <div v-else class="text-center py-6 text-gray-400 dark:text-gray-500 text-sm">
        今日暂无转写记录
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useMimoDashboardStore as useDashboardStore } from "@/stores/mimo-dashboard";
const store = useDashboardStore();

const tabs = [
  { value: "v25pro" as const, label: "V2.5 Pro" },
  { value: "v25" as const, label: "V2.5" },
  { value: "asr" as const, label: "V2.5 ASR" },
];

const activeTab = ref<"v25pro" | "v25" | "asr">("v25pro");

watch(activeTab, (value) => {
  if (value === "v25pro" || value === "v25") {
    store.selectedDetailModel = value;
  }
});

const detailData = computed(() => {
  return {
    cacheHit: store.detailTodayCacheHit,
    cacheMiss: store.detailTodayCacheMiss,
    output: store.detailTodayOutput,
    hitRate: store.detailTodayCacheHitRate,
  };
});

function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}秒`;
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  if (mins < 60) return secs > 0 ? `${mins}分${secs}秒` : `${mins}分`;
  const hours = Math.floor(mins / 60);
  const remainMins = mins % 60;
  return remainMins > 0 ? `${hours}时${remainMins}分` : `${hours}时`;
}
</script>
