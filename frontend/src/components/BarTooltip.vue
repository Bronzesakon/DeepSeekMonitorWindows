<template>
  <div
    v-if="visible"
    class="bar-tooltip"
    :style="tooltipStyle"
  >
    <div class="bar-tooltip-head">
      <span class="bar-tooltip-date">{{ data.date }}</span>
      <strong>{{ data.tokens.toLocaleString() }} tokens</strong>
    </div>
    <span class="bar-tooltip-row">
      <i class="dot hit" />缓存命中<strong>{{ data.cacheHit.toLocaleString() }} tokens</strong>
    </span>
    <span class="bar-tooltip-row">
      <i class="dot miss" />缓存未命中<strong>{{ data.cacheMiss.toLocaleString() }} tokens</strong>
    </span>
    <span class="bar-tooltip-row">
      <i class="dot response" />输出<strong>{{ data.output.toLocaleString() }} tokens</strong>
    </span>
    <span class="bar-tooltip-row" style="margin-top: 4px; padding-top: 4px; border-top: 1px solid rgba(255,255,255,0.1)">
      缓存命中<strong>{{ hitRate }}</strong>
    </span>
    <span class="bar-tooltip-row">
      费用<strong>¥{{ costFormatted }}</strong>
    </span>
    <span v-if="data.audioDuration > 0" class="bar-tooltip-row">
      音频转写<strong>{{ formatDuration(data.audioDuration) }}</strong>
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { ChartDataPoint } from "@/types/models";

const props = defineProps<{
  visible: boolean;
  x: number;
  y: number;
  data: ChartDataPoint;
  barIndex: number;
  totalBars: number;
}>();

const tooltipStyle = computed(() => {
  // Guess tooltip dimensions (will be refined after mount)
  const estW = 190;
  const estH = 160;
  const margin = 8;

  let left = props.x;
  let top = props.y - estH - margin;

  // Flip horizontally if would overflow right edge
  if (left + estW > window.innerWidth - margin) {
    left = Math.max(margin, window.innerWidth - estW - margin);
  }
  // Clamp left edge
  if (left < margin) left = margin;

  // Flip above if would overflow top
  if (top < margin) {
    top = props.y + margin;
  }

  return {
    left: left + "px",
    top: top + "px",
    position: "fixed" as const,
    zIndex: 1000,
  };
});

const alignClass = computed(() => {
  if (props.barIndex <= 1) return "align-left";
  if (props.barIndex >= props.totalBars - 2) return "align-right";
  return "";
});

const hitRate = computed(() => {
  const total = props.data.cacheHit + props.data.cacheMiss;
  return total > 0 ? `${((props.data.cacheHit / total) * 100).toFixed(2)}%` : "--";
});

const costFormatted = computed(() => (props.data.cost / 100).toFixed(2));

function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}秒`;
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  if (mins < 60) return secs > 0 ? `${mins}分${secs}秒` : `${mins}分`;
  const hours = Math.floor(mins / 60);
  const remainMins = mins % 60;
  return remainMins > 0 ? `${hours}时${remainMins}分` : `${hours}时`;
}
</script>

<style scoped>
.bar-tooltip {
  display: flex;
  flex-direction: column;
  gap: 3px;
  min-width: 120px;
  padding: 10px 12px;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.12);
  background: #2a2a2a;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
  font-size: 11px;
  font-weight: 700;
  line-height: 1.5;
  white-space: nowrap;
  pointer-events: none;
  color: rgba(246, 239, 222, 0.7);
}

.bar-tooltip-date {
  margin-bottom: 2px;
  color: rgba(246, 239, 222, 0.56);
  font-size: 10px;
  font-weight: 800;
}

.bar-tooltip strong {
  float: right;
  margin-left: 14px;
  color: rgba(246, 239, 222, 0.92);
}

.bar-tooltip-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
  margin-bottom: 7px;
  padding-bottom: 7px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.12);
}

.bar-tooltip-head .bar-tooltip-date {
  margin-bottom: 0;
  color: rgba(246, 239, 222, 0.95);
  font-size: 12px;
}

.bar-tooltip-head strong {
  float: none;
  margin-left: 0;
  font-size: 12px;
}

.bar-tooltip-row {
  display: flex;
  align-items: center;
  gap: 7px;
  font-weight: 600;
}

.bar-tooltip-row .dot {
  width: 8px;
  height: 8px;
  border-radius: 4px;
  flex: 0 0 auto;
}

.bar-tooltip-row .dot.hit { background: #34d399; }
.bar-tooltip-row .dot.miss { background: #D9A840; }
.bar-tooltip-row .dot.response { background: #a78bfa; }

.bar-tooltip-row strong {
  float: none;
  margin-left: auto;
  color: rgba(246, 239, 222, 0.95);
  font-weight: 800;
}

/* Light theme */
:root:not(.dark) .bar-tooltip {
  background: #ffffff;
  border-color: rgba(40, 70, 100, 0.14);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  color: rgba(22, 42, 64, 0.7);
}

:root:not(.dark) .bar-tooltip strong {
  color: rgba(22, 42, 64, 0.92);
}

:root:not(.dark) .bar-tooltip-head {
  border-bottom-color: rgba(40, 70, 100, 0.14);
}

:root:not(.dark) .bar-tooltip-head .bar-tooltip-date {
  color: rgba(22, 42, 64, 0.95);
}

:root:not(.dark) .bar-tooltip-row strong {
  color: rgba(22, 42, 64, 0.95);
}

:root:not(.dark) .bar-tooltip-date {
  color: rgba(22, 42, 64, 0.56);
}
</style>
