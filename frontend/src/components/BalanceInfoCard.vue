<template>
  <div class="glass-card p-5">
    <div class="flex items-start justify-between">
      <div class="text-sm font-semibold tracking-wider uppercase text-gray-500 dark:text-gray-400 mb-1">可用余额</div>
      <div class="flex items-center gap-1.5">
        <span
          class="inline-flex items-center gap-1 text-xs px-2 py-0.5 rounded-full font-medium backdrop-blur-sm border border-white/20 dark:border-white/10"
          :class="store.isAccountAvailable
            ? 'bg-green-500/10 text-green-700 dark:bg-green-500/15 dark:text-green-400'
            : 'bg-gray-500/10 text-gray-500 dark:text-gray-400'"
        >
          <span>{{ store.isAccountAvailable ? '✔' : '✖' }}</span>
          <span>{{ store.isAccountAvailable ? '可用' : '不可用' }}</span>
        </span>
        <button
          @click="invoke(openBalanceCmd)"
          class="inline-flex items-center justify-center gap-1 text-xs px-2 py-1 rounded-full font-medium bg-orange-500/10 text-orange-600 dark:text-orange-400 backdrop-blur-sm border border-white/20 dark:border-white/10 hover:bg-orange-500/20 transition-colors cursor-pointer"
        >
          <svg class="w-3 h-3 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="5" width="20" height="14" rx="2"/>
            <line x1="2" y1="10" x2="22" y2="10"/>
          </svg>
          <span class="leading-none">充值</span>
        </button>
      </div>
    </div>
    <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">¥{{ store.totalBalance.toFixed(2) }}</div>

    <div class="grid grid-cols-2 gap-3 mt-3">
      <div class="text-left p-2 rounded-lg border border-gray-200 dark:border-zinc-600 shadow-glass bg-gray-100 dark:bg-zinc-700">
        <div class="text-xs font-semibold tracking-wider uppercase text-gray-500 dark:text-gray-400">今日费用</div>
        <div class="text-xl font-bold text-orange-500 dark:text-orange-400">¥{{ store.currentDayCost.toFixed(2) }}</div>
      </div>
      <div class="text-left p-2 rounded-lg border border-gray-200 dark:border-zinc-600 shadow-glass bg-gray-100 dark:bg-zinc-700">
        <div class="text-xs font-semibold tracking-wider uppercase text-gray-500 dark:text-gray-400">本月费用</div>
        <div class="text-xl font-bold text-orange-500 dark:text-orange-400">¥{{ store.currentMonthCost.toFixed(2) }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useDashboardStore } from "@/stores/dashboard";
import { useMimoDashboardStore } from "@/stores/mimo-dashboard";
import { useProviderStore } from "@/stores/provider";
import { invoke } from "@tauri-apps/api/core";

const providerStore = useProviderStore();
const dsStore = useDashboardStore();
const mimoStore = useMimoDashboardStore();

const store = computed(() =>
  providerStore.activeProvider.value === "deepseek" ? dsStore : mimoStore
);

const openBalanceCmd = computed(() =>
  providerStore.activeProvider.value === "deepseek" ? "open_top_up_window" : "open_balance_page"
);
</script>
