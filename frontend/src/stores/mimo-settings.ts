import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";

export const useSettingsStore = defineStore("mimo-settings", () => {
  const hasPlatformSession = ref(false);
  const refreshInterval = ref(60);

  const edgeSnapEnabled = ref(true);
  const autoStart = ref(false);
  const autoStartPending = ref(false);

  const selectedDetailModel = ref<"v25pro" | "v25">("v25pro");
  const selectedTrendModel = ref<"v25pro" | "v25">("v25pro");
  const paymentMode = ref<"plan" | "token">("token");
  const modeSwitching = ref(false);

  async function openPlatformLogin() {
    await invoke("mimo_open_login_window");
  }

  async function clearPlatformSession() {
    await invoke("mimo_clear_platform_session");
    try {
      await invoke("plugin:core|clear_all_browsing_data");
    } catch (_) {}
    hasPlatformSession.value = false;
    invoke("mimo_broadcast_refresh");
  }

  function setRefreshInterval(interval: number) {
    console.log("[settings] 刷新间隔变更为 " + interval + " 秒", new Date().toLocaleTimeString());
    refreshInterval.value = interval;
    invoke("save_setting", { key: "refresh_interval", value: String(interval) });
  }

  async function loadRefreshInterval() {
    const val = await invoke<string | null>("load_setting", { key: "refresh_interval" });
    const parsed = val !== null ? parseInt(val, 10) : 60;
    if (!isNaN(parsed) && parsed > 0) {
      refreshInterval.value = parsed;
    }
  }

  async function loadEdgeSnapEnabled() {
    const val = await invoke<string | null>("load_setting", { key: "edge_snap_enabled" });
    edgeSnapEnabled.value = val === "true";
  }

  async function setEdgeSnapEnabled(enabled: boolean) {
    edgeSnapEnabled.value = enabled;
    await invoke("save_setting", { key: "edge_snap_enabled", value: String(enabled) });
    await invoke("broadcast_edge_snap_setting", { enabled });
  }

  async function loadSelectedDetailModel(): Promise<"v25pro" | "v25"> {
    const val = await invoke<string | null>("load_setting", { key: "selected_detail_model" });
    const result = (val === "v25" ? "v25" : "v25pro") as "v25pro" | "v25";
    selectedDetailModel.value = result;
    return result;
  }

  async function setSelectedDetailModel(value: "v25pro" | "v25") {
    if (selectedDetailModel.value === value) return;
    selectedDetailModel.value = value;
    await invoke("save_setting", { key: "selected_detail_model", value });
    await emit("detail-model-changed", value);
  }

  async function loadSelectedTrendModel(): Promise<"v25pro" | "v25"> {
    const val = await invoke<string | null>("load_setting", { key: "selected_trend_model" });
    const result = (val === "v25" ? "v25" : "v25pro") as "v25pro" | "v25";
    selectedTrendModel.value = result;
    return result;
  }

  async function setSelectedTrendModel(value: "v25pro" | "v25") {
    if (selectedTrendModel.value === value) return;
    selectedTrendModel.value = value;
    await invoke("save_setting", { key: "selected_trend_model", value });
  }

  async function loadPaymentMode(): Promise<"plan" | "token"> {
    const val = await invoke<string | null>("load_setting", { key: "payment_mode" });
    const result = (val === "token" ? "token" : "plan") as "plan" | "token";
    paymentMode.value = result;
    return result;
  }

  async function setPaymentMode(value: "plan" | "token") {
    if (paymentMode.value === value || modeSwitching.value) return;
    // 调用方窗口（SettingsView）立即更新按钮高亮，提供即时反馈
    // DashboardView/WidgetView 通过 payment-mode-changed 事件延迟更新，避免数据为空时闪烁
    modeSwitching.value = true;
    paymentMode.value = value;
    try {
      await invoke("save_setting", { key: "payment_mode", value });
      await invoke("broadcast_payment_mode", { mode: value });
    } finally {
      modeSwitching.value = false;
    }
  }

  async function loadAutoStart() {
    try {
      autoStart.value = await invoke<boolean>("is_autostart_enabled");
    } catch (_) {
      autoStart.value = false;
    }
  }

  async function setAutoStart(enabled: boolean) {
    if (autoStartPending.value) return;
    autoStartPending.value = true;
    const prev = autoStart.value;
    autoStart.value = enabled;
    try {
      await invoke(enabled ? "enable_autostart" : "disable_autostart");
    } catch (_) {
      autoStart.value = prev;
    } finally {
      autoStartPending.value = false;
    }
  }

  return {
    hasPlatformSession, refreshInterval, edgeSnapEnabled,
    selectedDetailModel, selectedTrendModel, paymentMode, modeSwitching,
    autoStart, autoStartPending,
    openPlatformLogin, clearPlatformSession,
    setRefreshInterval, loadRefreshInterval,
    loadEdgeSnapEnabled, setEdgeSnapEnabled,
    loadSelectedDetailModel, setSelectedDetailModel,
    loadSelectedTrendModel, setSelectedTrendModel,
    loadPaymentMode, setPaymentMode,
    loadAutoStart, setAutoStart,
  };
});