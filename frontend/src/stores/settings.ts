import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";

export const useSettingsStore = defineStore("settings", () => {
  const hasPlatformSession = ref(false);
  const refreshInterval = ref(60);

  const edgeSnapEnabled = ref(true); // default true, 避免首次渲染时展开/收起闪烁
  const autoStart = ref(false);
  const autoStartPending = ref(false); // 正在操作中，禁止重复点击

  // 模型单选按钮持久化
  const selectedDetailModel = ref<"pro" | "flash">("pro");
  const selectedTrendModel = ref<"pro" | "flash">("pro");

  async function openPlatformLogin() {
    await invoke("ds_open_login_window");
  }

  async function clearPlatformSession() {
    await invoke("ds_clear_platform_session");
    try {
      await invoke("plugin:core|clear_all_browsing_data");
    } catch (_) {}
    hasPlatformSession.value = false;
    invoke("ds_broadcast_refresh");
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

  async function loadSelectedDetailModel(): Promise<"pro" | "flash"> {
    const val = await invoke<string | null>("load_setting", { key: "selected_detail_model" });
    const result = (val === "flash" ? "flash" : "pro") as "pro" | "flash";
    selectedDetailModel.value = result;
    return result;
  }

  async function setSelectedDetailModel(value: "pro" | "flash") {
    if (selectedDetailModel.value === value) return;
    selectedDetailModel.value = value;
    await invoke("save_setting", { key: "selected_detail_model", value });
    await emit("detail-model-changed", value);
  }

  async function loadSelectedTrendModel(): Promise<"pro" | "flash"> {
    const val = await invoke<string | null>("load_setting", { key: "selected_trend_model" });
    const result = (val === "flash" ? "flash" : "pro") as "pro" | "flash";
    selectedTrendModel.value = result;
    return result;
  }

  async function setSelectedTrendModel(value: "pro" | "flash") {
    if (selectedTrendModel.value === value) return;
    selectedTrendModel.value = value;
    await invoke("save_setting", { key: "selected_trend_model", value });
  }

  async function loadAutoStart() {
    try {
      autoStart.value = await invoke<boolean>("get_auto_start");
    } catch (_) {
      autoStart.value = false;
    }
  }

  async function setAutoStart(enabled: boolean) {
    if (autoStartPending.value) return;
    autoStartPending.value = true;
    // 乐观更新：立即切换按钮，不等待后端
    const prev = autoStart.value;
    autoStart.value = enabled;
    try {
      await invoke("set_auto_start", { enabled });
    } catch (_) {
      // 后端验证失败，回退到之前的状态
      autoStart.value = prev;
    } finally {
      autoStartPending.value = false;
    }
  }

  return {
    hasPlatformSession, refreshInterval, edgeSnapEnabled,
    selectedDetailModel, selectedTrendModel,
    autoStart, autoStartPending,
    openPlatformLogin, clearPlatformSession,
    setRefreshInterval, loadRefreshInterval,
    loadEdgeSnapEnabled, setEdgeSnapEnabled,
    loadSelectedDetailModel, setSelectedDetailModel,
    loadSelectedTrendModel, setSelectedTrendModel,
    loadAutoStart, setAutoStart,
  };
});