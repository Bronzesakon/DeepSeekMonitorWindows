<template>
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useThemeStore } from "./stores/theme";
import { useProviderStore } from "./stores/provider";
import { invoke } from "@tauri-apps/api/core";

const theme = useThemeStore();
theme.init();

const provider = useProviderStore();
provider.init();

// Win version: default to Win11 (rounded) immediately, correct if backend says Win10
document.documentElement.dataset.winVersion = "11";
document.body.dataset.winVersion = "11";

onMounted(async () => {
  try {
    const isWin11 = await invoke<boolean>("is_windows11");
    const ver = isWin11 ? "11" : "10";
    document.documentElement.dataset.winVersion = ver;
    document.body.dataset.winVersion = ver;
  } catch {
    // keep "11" fallback
  }
});
</script>
