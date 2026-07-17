<template>
  <div class="glass-card p-5">
    <div class="grid grid-cols-2 gap-4">
      <div class="text-center p-2 rounded-lg bg-white/30 dark:bg-zinc-900/30 backdrop-blur-sm">
        <div class="text-[11px] font-medium tracking-wider uppercase text-gray-500 dark:text-gray-400 mb-1">总余额</div>
        <div class="text-xl font-bold text-gray-800 dark:text-gray-100">¥{{ store.totalBalance.toFixed(2) }}</div>
      </div>
      <div class="text-center p-2 rounded-lg bg-white/30 dark:bg-zinc-900/30 backdrop-blur-sm">
        <div class="text-[11px] font-medium tracking-wider uppercase text-gray-500 dark:text-gray-400 mb-1">可用余额</div>
        <div class="text-xl font-bold text-gray-800 dark:text-gray-100">¥{{ store.grantedBalance.toFixed(2) }}</div>
      </div>
    </div>

    <div class="flex justify-center mt-3">
      <div
        class="inline-flex items-center gap-1 text-xs px-2 py-0.5 rounded-full font-medium backdrop-blur-sm border border-white/20 dark:border-white/10"
        :class="store.isAccountAvailable
          ? 'bg-green-500/10 dark:bg-green-500/15 text-green-700 dark:text-green-400'
          : 'bg-gray-500/10 text-gray-500 dark:text-gray-400'"
      >
        <span>{{ store.isAccountAvailable ? '✔' : '✖' }}</span>
        <span>{{ store.isAccountAvailable ? '可用' : '不可用' }}</span>
      </div>
    </div>

    <div class="grid grid-cols-2 gap-3 mt-4">
      <div class="text-center p-2 rounded-lg bg-white/30 dark:bg-zinc-900/30 backdrop-blur-sm">
        <div class="text-[11px] font-medium tracking-wider uppercase text-gray-500 dark:text-gray-400">今日费用</div>
        <div class="text-xl font-bold text-gray-800 dark:text-gray-100">¥{{ store.currentDayCost.toFixed(4) }}</div>
      </div>
      <div class="text-center p-2 rounded-lg bg-white/30 dark:bg-zinc-900/30 backdrop-blur-sm">
        <div class="text-[11px] font-medium tracking-wider uppercase text-gray-500 dark:text-gray-400">本月费用</div>
        <div class="text-xl font-bold text-gray-800 dark:text-gray-100">¥{{ store.currentMonthCost.toFixed(4) }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useDashboardStore } from "@/stores/dashboard";
const store = useDashboardStore();
</script>
