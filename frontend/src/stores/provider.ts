import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export type Provider = "deepseek" | "mimo";

const activeProvider = ref<Provider>("deepseek");

export function useProviderStore() {
  async function init() {
    try {
      const val = await invoke<string | null>("load_setting", { key: "active_provider" });
      if (val === "mimo") activeProvider.value = "mimo";
    } catch {}
  }

  function setProvider(p: Provider) {
    activeProvider.value = p;
    invoke("save_setting", { key: "active_provider", value: p });
  }

  function toggle() {
    setProvider(activeProvider.value === "deepseek" ? "mimo" : "deepseek");
  }

  return { activeProvider, init, setProvider, toggle };
}
