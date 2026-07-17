<template>
  <div class="min-h-[500px] flex flex-col justify-center">
    <div class="space-y-3">
      <div class="glass-card p-5 flex flex-col justify-center" style="min-height: 300px">
        <div class="text-center mb-6">
          <h2 class="text-base font-bold text-gray-800 dark:text-gray-100">
            欢迎使用 DeepSeek 桌面助手
          </h2>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1 mb-10">
            登录 DeepSeek Platform 获取余额和用量数据
          </p>
        </div>
        <button
          @click="startLogin"
          :disabled="isLoggingIn"
          class="w-full py-2.5 bg-brand-500 hover:bg-brand-600 disabled:opacity-60 text-white text-sm font-bold rounded-lg transition shadow-md shadow-brand-500/30 flex items-center justify-center gap-2"
        >
          <svg v-if="isLoggingIn" class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
          {{ isLoggingIn ? '登录中...' : '打开登录页面' }}
        </button>
      </div>

      <ErrorBanner
        v-if="onboardingError"
        :error="onboardingError"
        :warning="null"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { useDashboardStore } from "@/stores/dashboard";
import ErrorBanner from "@/components/ErrorBanner.vue";

const dashboard = useDashboardStore();
const isLoggingIn = ref(false);
const onboardingError = ref<string | null>(null);

let unlistenComplete: UnlistenFn | null = null;
let unlistenCancelled: UnlistenFn | null = null;

// 监听 store 错误消息变化，保持响应式同步
watch(() => dashboard.errorMessage, (newMsg) => {
  if (newMsg) {
    onboardingError.value = newMsg;
  }
});

onMounted(async () => {
  if (dashboard.errorMessage) {
    onboardingError.value = dashboard.errorMessage;
  }

  // 主动刷新以即时检测 token/cookie 失效（必须等待完成，避免与 login-complete 冲突）
  await dashboard.refresh();
  if (dashboard.errorMessage) {
    onboardingError.value = dashboard.errorMessage;
  }

  unlistenComplete = await listen("ds-login-complete", async () => {
    isLoggingIn.value = false;
    dashboard.hasPlatformSession = true;
    dashboard.isFirstLaunch = false;
    onboardingError.value = null;
    await invoke("ds_broadcast_refresh");
  });

  unlistenCancelled = await listen("ds-login-cancelled", () => {
    isLoggingIn.value = false;
    dashboard.hasPlatformSession = false;
  });
});

onUnmounted(() => {
  if (unlistenComplete) unlistenComplete();
  if (unlistenCancelled) unlistenCancelled();
});

async function startLogin() {
  isLoggingIn.value = true;
  try {
    await invoke("ds_open_login_window");
  } catch (e) {
    console.error("打开登录窗口失败:", e);
    isLoggingIn.value = false;
  }
}
</script>
