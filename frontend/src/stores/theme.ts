import { defineStore } from "pinia";
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

export type ThemeMode = "light" | "dark" | "system";

const THEME_KEY = "app_theme";

function resolveTheme(mode: ThemeMode): "dark" | "light" {
  if (mode === "system") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return mode;
}

function applyDarkClass(isDark: boolean) {
  if (isDark) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
}

export const useThemeStore = defineStore("theme", () => {
  const isDark = ref(false);
  const mode = ref<ThemeMode>("light");

  let systemMq: MediaQueryList | null = null;
  let onSystemChange: ((e: MediaQueryListEvent) => void) | null = null;

  function init() {
    invoke<string | null>("load_setting", { key: THEME_KEY }).then((theme) => {
      if (theme === "dark") {
        mode.value = "dark";
        isDark.value = true;
        applyDarkClass(true);
      } else if (theme === "system") {
        mode.value = "system";
        const prefers = window.matchMedia("(prefers-color-scheme: dark)").matches;
        isDark.value = prefers;
        applyDarkClass(prefers);
        setupSystemListener();
      } else {
        mode.value = "light";
        isDark.value = false;
        applyDarkClass(false);
      }
    });

    // Sync theme across all windows
    listen<string>("theme-mode-changed", (event) => {
      const newMode = event.payload as ThemeMode;
      if (mode.value !== newMode) {
        mode.value = newMode;
        if (newMode === "system") {
          const prefers = window.matchMedia("(prefers-color-scheme: dark)").matches;
          isDark.value = prefers;
          applyDarkClass(prefers);
          setupSystemListener();
        } else {
          teardownSystemListener();
          const dark = newMode === "dark";
          isDark.value = dark;
          applyDarkClass(dark);
        }
      }
    });
  }

  function setupSystemListener() {
    if (systemMq) return;
    systemMq = window.matchMedia("(prefers-color-scheme: dark)");
    onSystemChange = (e: MediaQueryListEvent) => {
      if (mode.value === "system") {
        isDark.value = e.matches;
        applyDarkClass(e.matches);
      }
    };
    systemMq.addEventListener("change", onSystemChange);
  }

  function teardownSystemListener() {
    if (systemMq && onSystemChange) {
      systemMq.removeEventListener("change", onSystemChange);
      systemMq = null;
      onSystemChange = null;
    }
  }

  watch(isDark, (newVal) => {
    applyDarkClass(newVal);
    // Don't save to disk in system mode — the system change events handle appearance
  });

  function setMode(newMode: ThemeMode) {
    mode.value = newMode;
    if (newMode === "system") {
      const prefers = window.matchMedia("(prefers-color-scheme: dark)").matches;
      isDark.value = prefers;
      applyDarkClass(prefers);
      setupSystemListener();
      invoke("save_setting", { key: THEME_KEY, value: "system" });
    } else {
      teardownSystemListener();
      const dark = newMode === "dark";
      isDark.value = dark;
      applyDarkClass(dark);
      invoke("save_setting", { key: THEME_KEY, value: newMode });
    }
    emit("theme-mode-changed", newMode);
  }

  function toggle() {
    // Toggle between light and dark, skipping system
    setMode(mode.value === "dark" ? "light" : "dark");
  }

  return { isDark, mode, init, toggle, setMode };
});
