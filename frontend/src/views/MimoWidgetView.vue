<template>
  <div ref="rootEl" :class="['px-5 pt-2 glass-root', collapsed ? '' : 'pb-4']" @mouseenter="onTitleEnter" @mouseleave="onRootLeave">
      <div data-tauri-drag-region :class="collapsed ? 'relative flex items-center justify-center mb-2' : 'title-bar mb-2'" @mouseenter="onTitleEnter" @mousedown="onTitleDown">
        <ProviderToggle v-if="!collapsed" />
        <span v-if="collapsed" class="inline-flex items-center text-sm font-semibold">
          <template v-if="store.hasPlatformSession">
            <template v-if="settings.paymentMode === 'plan'">
              <span class="text-sm text-gray-600 dark:text-gray-300">套餐</span>
              <span class="text-sm text-orange-500 dark:text-orange-400 ml-1">{{ (store.planUsagePercent ?? 0).toFixed(1) }}%</span>
            </template>
            <template v-else>
              <span class="text-sm text-gray-600 dark:text-gray-300">今日费用</span>
              <span class="text-sm text-orange-500 dark:text-orange-400 ml-1">¥{{ store.currentDayCost.toFixed(2) }}</span>
            </template>
            <span class="w-px h-3 bg-gray-400 dark:bg-gray-400 mx-1.5"></span><span class="text-sm text-gray-600 dark:text-gray-300">{{ settings.selectedDetailModel === 'v25pro' ? 'V2.5 Pro' : 'V2.5' }}</span>
            <span class="text-sm text-blue-600 dark:text-blue-400 ml-1">{{ (settings.selectedDetailModel === 'v25pro' ? store.currentDayProTokens : store.currentDayFlashTokens).toLocaleString() }}</span>
            <span class="w-px h-3 bg-gray-400 dark:bg-gray-400 mx-1.5"></span><span class="text-sm text-gray-600 dark:text-gray-300">命中率</span>
            <span class="text-sm text-purple-400 dark:text-purple-400 ml-1">{{ store.detailTodayCacheHitRate }}</span>
          </template>
          <span v-else-if="!store.hasPlatformSession" class="text-gray-500 dark:text-gray-400">未登录 Platform</span>
        </span>
        <div v-if="!collapsed" class="title-actions">
          <button aria-label="刷新" class="title-btn" @click="store.refresh()"><RefreshIcon /></button>
          <button aria-label="主题" class="title-btn" @click="theme.toggle"><ThemeIcon /></button>
          <button aria-label="设置" class="title-btn" @click="openSettings"><SettingsIcon /></button>
          <button aria-label="关闭" class="title-btn close-btn" @click="closeWindow"><CloseIcon /></button>
        </div>
      </div>
      <div ref="contentWrap" v-show="!collapsed" class="space-y-3">

      <div v-if="!store.hasLoaded">
      </div>

      <template v-else-if="store.isFirstLaunch || !store.hasPlatformSession">
      <div class="glass-card p-5">
        <div class="text-center mb-3">
          <div class="text-2xl mb-2">{{ store.isFirstLaunch ? '👋' : '⚠️' }}</div>
          <div class="text-sm font-semibold text-gray-700 dark:text-gray-300">{{ store.isFirstLaunch ? '欢迎使用' : '需要登录' }}</div>
          <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">请在主界面完成登录配置</div>
        </div>
        <button @click="openMainWindow" class="w-full px-3 py-2 bg-brand-500 hover:bg-brand-600 text-white text-xs font-semibold rounded-lg transition shadow-sm shadow-brand-500/20">
          打开主界面
        </button>
      </div>
      <ErrorBanner
        v-if="store.errorMessage && !store.isFirstLaunch"
        class="mt-3"
        :error="store.errorMessage"
        :warning="null"
      />
      </template>

      <template v-else>
      <ErrorBanner
        :error="store.errorMessage"
        :warning="store.warningMessage"
        :showSettings="true"
      />

      <div v-if="settings.paymentMode === 'plan'" class="glass-card p-5">
        <div class="flex items-start justify-between mb-2">
          <div>
            <div class="text-sm font-semibold tracking-wider uppercase text-gray-500 dark:text-gray-400 mb-1">当前套餐</div>
            <div v-if="store.planName" class="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {{ store.planName }}
            </div>
            <div v-else class="text-lg font-semibold text-gray-400 dark:text-gray-500">未订阅</div>
          </div>
          <div class="flex items-center gap-1.5">
            <span v-if="store.planName" class="inline-flex items-center gap-1 text-xs px-2 py-0.5 rounded-full font-medium backdrop-blur-sm border border-white/20 dark:border-white/10" :class="store.planExpired ? 'bg-gray-500/10 text-gray-500 dark:text-gray-400' : 'bg-green-500/10 text-green-700 dark:bg-green-500/15 dark:text-green-400'">
              <span>{{ store.planExpired ? '✖' : '✔' }}</span>
              <span>{{ store.planExpired ? '已过期' : '有效' }}</span>
            </span>
            <span v-else class="inline-flex items-center gap-1 text-xs px-2 py-0.5 rounded-full font-medium bg-gray-500/10 text-gray-500 dark:text-gray-400 backdrop-blur-sm border border-white/20 dark:border-white/10">
              <span>—</span>
              <span>未订阅</span>
            </span>
            <button @click="invoke('open_plan_manage')" class="inline-flex items-center justify-center gap-1 text-xs px-2 py-1 rounded-full font-medium bg-orange-500/10 text-orange-600 dark:text-orange-400 backdrop-blur-sm border border-white/20 dark:border-white/10 hover:bg-orange-500/20 transition-colors cursor-pointer">
              <span class="leading-none">续费</span>
            </button>
          </div>
        </div>
        <div v-if="store.planName">
          <div class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400 mb-1.5">
            <span>套餐用量</span>
            <span class="tabular-nums">
              <template v-if="store.planUsed != null && store.planLimit != null">
                {{ store.planUsed.toLocaleString() }} / {{ store.planLimit.toLocaleString() }}
              </template>
              <template v-else>– / –</template>
              已使用 {{ store.planUsagePercent != null ? (store.planUsagePercent).toFixed(1) + '%' : '–' }}
            </span>
          </div>
          <div class="w-full h-3 rounded-full bg-gray-200 dark:bg-zinc-700 overflow-hidden">
            <div class="h-full rounded-full transition-all duration-500" :style="{ width: Math.min(store.planUsagePercent ?? 0, 100) + '%' }" :class="(store.planUsagePercent ?? 0) > 90 ? 'bg-red-500 dark:bg-red-400' : (store.planUsagePercent ?? 0) > 70 ? 'bg-orange-400 dark:bg-orange-400' : 'bg-blue-600 dark:bg-blue-400'"></div>
          </div>
          <div v-if="store.planPeriodEnd" class="text-xs text-gray-400 dark:text-gray-500 mt-2">
            到期时间：{{ store.planPeriodEnd }}
          </div>
        </div>
      </div>

      <BalanceInfoCard v-else />

      <TokenDetail />
      </template>
      </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch, nextTick } from "vue";
import { getCurrentWindow, monitorFromPoint } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/dpi";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useMimoDashboardStore as useDashboardStore } from "@/stores/mimo-dashboard";
import { useThemeStore } from "@/stores/theme";
import { useSettingsStore } from "@/stores/mimo-settings";
import TokenDetail from "@/components/TokenDetail.vue";
import ErrorBanner from "@/components/ErrorBanner.vue";
import BalanceInfoCard from "@/components/BalanceInfoCard.vue";
import ProviderToggle from "@/components/ProviderToggle.vue";
import RefreshIcon from "@/components/icons/RefreshIcon.vue";
import ThemeIcon from "@/components/icons/ThemeIcon.vue";
import SettingsIcon from "@/components/icons/SettingsIcon.vue";
import CloseIcon from "@/components/icons/CloseIcon.vue";

const store = useDashboardStore();
const theme = useThemeStore();
const settings = useSettingsStore();
const currentLabel = getCurrentWindow().label;
const rootEl = ref<HTMLElement>();
const collapsed = ref(false);
const docked = ref(false);

watch(() => store.selectedDetailModel, (val) => { settings.setSelectedDetailModel(val); });
watch(() => store.selectedTrendModel, (val) => { settings.setSelectedTrendModel(val); });

let dragGuard = false;
let dragGuardTimer: ReturnType<typeof setTimeout> | null = null;
let resizeObserver: ResizeObserver | null = null;
let unlistenSnap: (() => void) | null = null;
let unlistenRelease: (() => void) | null = null;

function setupResizeObserver() {
  if (!rootEl.value) return;
  if (resizeObserver) resizeObserver.disconnect();
  let pending = false;
  const resize = () => {
    const h = rootEl.value!.scrollHeight;
    if (h > 0) getCurrentWindow().setSize(new LogicalSize(400, h)).catch(() => {});
  };
  resize();
  resizeObserver = new ResizeObserver(() => {
    if (pending) return;
    pending = true;
    requestAnimationFrame(() => {
      pending = false;
      resize();
    });
  });
  resizeObserver.observe(rootEl.value);
}

function clearDragGuard() {
  dragGuard = false;
  if (dragGuardTimer) { clearTimeout(dragGuardTimer); dragGuardTimer = null; }
}

function onTitleEnter(e: MouseEvent) {
  if (dragGuard) return;
  if (e.clientX < 0 || e.clientX >= window.innerWidth) return;
  if (e.clientY < 0 || e.clientY >= window.innerHeight) return;
  if (docked.value && collapsed.value && settings.edgeSnapEnabled) {
    collapsed.value = false;
  }
}

function onTitleDown() {
  dragGuard = true;
  if (dragGuardTimer) clearTimeout(dragGuardTimer);
  dragGuardTimer = setTimeout(() => { dragGuard = false; }, 2000);
}

function onRootLeave() {
  if (dragGuard) return;
  if (docked.value && !collapsed.value && settings.edgeSnapEnabled) {
    collapsed.value = true;
  }
}

onMounted(async () => {
  const cached = await invoke<any | null>("mimo_get_cached_data").catch(() => null);
  if (cached) store.applyData(cached);
  else store.refresh();
  listen("detail-model-changed", (event: { payload: "v25pro" | "v25" }) => {
    store.selectedDetailModel = event.payload;
  });
  listen("mimo-login-complete", () => { invoke("mimo_broadcast_refresh"); });
  listen("mimo-trigger-refresh", (event: { payload: any }) => {
    if (event.payload && typeof event.payload === 'object' && 'has_platform_session' in event.payload) {
      store.applyData(event.payload);
    } else {
      store.refresh();
    }
  });
  listen("edge-snap-setting-changed", (event: { payload: boolean }) => {
    settings.edgeSnapEnabled = event.payload;
    if (event.payload) {
      getCurrentWindow().outerPosition().then(async pos => {
        const monitor = await monitorFromPoint(pos.x, pos.y);
        const monitorTop = monitor?.position.y ?? 0;
        const relY = pos.y - monitorTop;
        if (relY <= 10 && !collapsed.value) {
          docked.value = true;
          collapsed.value = true;
        }
      }).catch(() => {});
    } else if (docked.value) {
      docked.value = false;
      if (collapsed.value) collapsed.value = false;
    }
  });
  listen("payment-mode-changed", (event: { payload: "plan" | "token" }) => {
    settings.paymentMode = event.payload;
    // 不调用 store.refresh()，数据已由 broadcast_payment_mode 中的 trigger-refresh 事件提供
  });

  settings.loadPaymentMode();
  settings.loadSelectedDetailModel().then(val => { store.selectedDetailModel = val; });
  settings.loadSelectedTrendModel().then(val => { store.selectedTrendModel = val; });

  const [_snap, unlistenSnapFn, unlistenReleaseFn] = await Promise.all([
    settings.loadEdgeSnapEnabled(),
    getCurrentWindow().listen<{ label: string }>("edge-snap-triggered", (event) => {
      if (event.payload.label !== currentLabel) return;
      clearDragGuard();
      if (settings.edgeSnapEnabled) {
        docked.value = true;
        collapsed.value = true;
      }
    }),
    getCurrentWindow().listen<{ label: string }>("edge-snap-released", (event) => {
      if (event.payload.label !== currentLabel) return;
      clearDragGuard();
      docked.value = false;
      if (collapsed.value) collapsed.value = false;
    }),
  ]);
  unlistenSnap = unlistenSnapFn;
  unlistenRelease = unlistenReleaseFn;

  if (settings.edgeSnapEnabled) {
    try {
      const pos = await getCurrentWindow().outerPosition();
      const monitor = await monitorFromPoint(pos.x, pos.y);
      const monitorTop = monitor?.position.y ?? 0;
      const relY = pos.y - monitorTop;
      if (relY <= 10) {
        docked.value = true;
        collapsed.value = true;
      }
    } catch (_) {}
  }

  await nextTick();
  setupResizeObserver();
});

onUnmounted(() => {
  if (resizeObserver) resizeObserver.disconnect();
  if (dragGuardTimer) clearTimeout(dragGuardTimer);
  if (unlistenSnap) unlistenSnap();
  if (unlistenRelease) unlistenRelease();
});

function closeWindow() {
  invoke("hide_widget_window");
}

function openSettings() {
  invoke("show_settings_window");
}

function openMainWindow() {
  invoke("show_main_window");
}
</script>
