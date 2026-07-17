import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { MimoDashboardData as DashboardData, MimoModelDailyUsagePoint as ModelDailyUsagePoint, MimoModelUsageSummary as ModelUsageSummary } from "@/types/models";
import { useSettingsStore } from "@/stores/mimo-settings";

function formatLocalDate(d: Date): string {
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}

export const useMimoDashboardStore = defineStore("mimo-dashboard", () => {
  const isAccountAvailable = ref(false);
  const totalBalance = ref(0);
  const grantedBalance = ref(0);
  const toppedUpBalance = ref(0);

  const flashUsage = ref<ModelUsageSummary | null>(null);
  const proUsage = ref<ModelUsageSummary | null>(null);
  const flashDailyUsage = ref<ModelDailyUsagePoint[]>([]);
  const proDailyUsage = ref<ModelDailyUsagePoint[]>([]);

  const currentDayCost = ref(0);
  const currentMonthCost = ref(0);
  const currentDayRequests = ref(0);
  const currentDayFlashTokens = ref(0);
  const currentDayProTokens = ref(0);
  const currentDayAudioDuration = ref(0);
  const planName = ref<string | null>(null);
  const planExpired = ref<boolean | null>(null);
  const planPeriodEnd = ref<string | null>(null);
  const planUsagePercent = ref<number | null>(null);
  const planUsed = ref<number | null>(null);
  const planLimit = ref<number | null>(null);
  const hasPlatformSession = ref(false);
  const isFirstLaunch = ref(false);
  const isLoading = ref(false);
  const hasLoaded = ref(false);
  const lastUpdated = ref("");
  const errorMessage = ref<string | null>(null);
  const warningMessage = ref<string | null>(null);

  const selectedTrendModel = ref<"v25pro" | "v25">("v25pro");
  const selectedDetailModel = ref<"v25pro" | "v25">("v25pro");
  const selectedWidgetModel = ref<"v25pro" | "v25">("v25pro");

  const nowTime = ref(Date.now());
  setInterval(() => { nowTime.value = Date.now(); }, 60_000);

  const lastSevenDatesList = computed((): string[] => {
    const dates: string[] = [];
    const now = new Date(nowTime.value);
    const hour = now.getHours();
    const baseDate = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    if (hour < 7) baseDate.setDate(baseDate.getDate() - 1);
    for (let i = 6; i >= 0; i--) {
      const d = new Date(baseDate);
      d.setDate(d.getDate() - i);
      dates.push(formatLocalDate(d));
    }
    return dates;
  });

  const todayStr = computed((): string => {
    const now = new Date(nowTime.value);
    const effective = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    if (now.getHours() < 7) effective.setDate(effective.getDate() - 1);
    return formatLocalDate(effective);
  });

  function toLabel(dateStr: string): string {
    return dateStr.length >= 10 ? dateStr.slice(5, 10) : dateStr;
  }

  const trendChartData = computed(() => {
    const source =
      selectedTrendModel.value === "v25pro" ? proDailyUsage.value : flashDailyUsage.value;
    const sourceMap = new Map(source.map((p) => [p.date, p]));
    return lastSevenDatesList.value.map((date) => {
      const p = sourceMap.get(date);
      return {
        label: toLabel(date),
        tokens: p?.total_tokens ?? 0,
        cost: p?.cost_in_cents ?? 0,
        cacheHit: p?.input_cache_hit_tokens ?? 0,
        cacheMiss: p?.input_cache_miss_tokens ?? 0,
        output: p?.output_tokens ?? 0,
        audioDuration: p?.input_audio_duration ?? 0,
        date,
      };
    });
  });

  const trendTodayPoint = computed(() => {
    const source =
      selectedTrendModel.value === "v25pro" ? proDailyUsage.value : flashDailyUsage.value;
    return source.find((p) => p.date === todayStr.value) || null;
  });

  const trendTodayCacheHit = computed(() => trendTodayPoint.value?.input_cache_hit_tokens ?? 0);
  const trendTodayCacheMiss = computed(() => trendTodayPoint.value?.input_cache_miss_tokens ?? 0);
  const trendTodayOutput = computed(() => trendTodayPoint.value?.output_tokens ?? 0);
  const trendTodayCacheHitRate = computed(() => {
    const total = trendTodayCacheHit.value + trendTodayCacheMiss.value;
    return total > 0 ? `${((trendTodayCacheHit.value / total) * 100).toFixed(2)}%` : "--";
  });

  const detailTodayPoint = computed(() => {
    const source =
      selectedDetailModel.value === "v25pro" ? proDailyUsage.value : flashDailyUsage.value;
    return source.find((p) => p.date === todayStr.value) || null;
  });

  const detailTodayCacheHit = computed(() => detailTodayPoint.value?.input_cache_hit_tokens ?? 0);
  const detailTodayCacheMiss = computed(() => detailTodayPoint.value?.input_cache_miss_tokens ?? 0);
  const detailTodayOutput = computed(() => detailTodayPoint.value?.output_tokens ?? 0);
  const detailTodayCacheHitRate = computed(() => {
    const total = detailTodayCacheHit.value + detailTodayCacheMiss.value;
    return total > 0 ? `${((detailTodayCacheHit.value / total) * 100).toFixed(2)}%` : "--";
  });
  const detailTodayAudioDuration = computed(() => detailTodayPoint.value?.input_audio_duration ?? 0);

  const widgetTodayPoint = computed(() => {
    const source =
      selectedWidgetModel.value === "v25pro" ? proDailyUsage.value : flashDailyUsage.value;
    return source.find((p) => p.date === todayStr.value) || null;
  });

  const widgetTodayCacheHit = computed(() => widgetTodayPoint.value?.input_cache_hit_tokens ?? 0);
  const widgetTodayCacheMiss = computed(() => widgetTodayPoint.value?.input_cache_miss_tokens ?? 0);
  const widgetTodayOutput = computed(() => widgetTodayPoint.value?.output_tokens ?? 0);
  const widgetTodayCacheHitRate = computed(() => {
    const total = widgetTodayCacheHit.value + widgetTodayCacheMiss.value;
    return total > 0 ? `${((widgetTodayCacheHit.value / total) * 100).toFixed(2)}%` : "--";
  });

  function applyData(data: DashboardData) {
    isAccountAvailable.value = data.is_account_available;
    totalBalance.value = data.total_balance;
    grantedBalance.value = data.granted_balance;
    toppedUpBalance.value = data.topped_up_balance;

    flashUsage.value = data.flash_usage;
    proUsage.value = data.pro_usage;
    flashDailyUsage.value = data.flash_daily_usage;
    proDailyUsage.value = data.pro_daily_usage;

    currentDayCost.value = data.current_day_cost;
    currentMonthCost.value = data.current_month_cost;
    currentDayRequests.value = data.current_day_requests;
    currentDayFlashTokens.value = data.current_day_flash_tokens;
    currentDayProTokens.value = data.current_day_pro_tokens;
    currentDayAudioDuration.value = data.current_day_audio_duration;

    planName.value = data.plan_name;
    planExpired.value = data.plan_expired;
    planPeriodEnd.value = data.plan_period_end;
    planUsagePercent.value = data.plan_usage_percent;
    planUsed.value = data.plan_used;
    planLimit.value = data.plan_limit;

    hasPlatformSession.value = data.has_platform_session;
    isFirstLaunch.value = data.is_first_launch;
    lastUpdated.value = data.last_updated;
    errorMessage.value = data.error_message;
    warningMessage.value = data.warning_message;
    // 数据与模式同步切换：trigger-refresh 携带 payment_mode，applyData 时立即更新模式
    // 避免先 applyData（数据跳变）后切模式（UI 切换）导致的两事件间闪烁
    const settings = useSettingsStore();
    const newMode = data.payment_mode === "token" ? "token" : "plan";
    if (settings.paymentMode !== newMode) {
      settings.paymentMode = newMode;
    }
    hasLoaded.value = true;
  }

  async function refresh() {
    console.log("[refresh] 主动请求 do_refresh...", new Date().toLocaleTimeString());
    isLoading.value = true;
    errorMessage.value = null;
    warningMessage.value = null;
    try {
      const data = await invoke<DashboardData>("mimo_refresh_data");
      console.log("[refresh] do_refresh 成功", new Date().toLocaleTimeString());
      applyData(data);
    } catch (e: any) {
      console.log("[refresh] do_refresh 失败:", e, new Date().toLocaleTimeString());
      if (e !== "refresh_in_progress") {
        errorMessage.value = typeof e === "string" ? e : e?.message || "未知错误";
      }
      hasLoaded.value = true;
    } finally {
      isLoading.value = false;
    }
  }

  return {
    isAccountAvailable, totalBalance, grantedBalance, toppedUpBalance,
    flashUsage, proUsage, flashDailyUsage, proDailyUsage,
    currentDayCost, currentMonthCost, currentDayRequests, currentDayFlashTokens, currentDayProTokens, currentDayAudioDuration,
    planName, planExpired, planPeriodEnd, planUsagePercent, planUsed, planLimit,
    hasPlatformSession, isFirstLaunch, isLoading, hasLoaded, lastUpdated, errorMessage, warningMessage,
    selectedTrendModel, selectedDetailModel, selectedWidgetModel,
    trendChartData,
    trendTodayCacheHit, trendTodayCacheMiss, trendTodayOutput, trendTodayCacheHitRate,
    detailTodayCacheHit, detailTodayCacheMiss, detailTodayOutput, detailTodayCacheHitRate, detailTodayAudioDuration,
    widgetTodayCacheHit, widgetTodayCacheMiss, widgetTodayOutput, widgetTodayCacheHitRate,
    applyData, refresh,
  };
});