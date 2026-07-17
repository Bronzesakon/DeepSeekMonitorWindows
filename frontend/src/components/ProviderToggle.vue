<template>
  <button class="provider-toggle" @click="toggle">
    <img v-if="activeProvider === 'deepseek'" src="/deepseek-color.svg" class="provider-icon" />
    <img v-else src="/mimo-icon.png" class="provider-icon" />
    <span class="provider-label">{{ activeProvider === 'deepseek' ? 'DeepSeek' : 'MiMo' }}</span>
    <span class="provider-arrow">⇄</span>
  </button>
</template>

<script setup lang="ts">
import { useProviderStore } from "@/stores/provider";

const { activeProvider, toggle } = useProviderStore();
</script>
