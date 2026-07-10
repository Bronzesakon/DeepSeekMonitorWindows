import React from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import "./styles.css";

import type { ViewName, ModelName, Provider, AppConfig, BalanceData, MimoBalanceData, BalanceState, UsageResult, MimoUsageResult } from "./types";
import { addDays, previousMonth, fetchWithCache } from "./utils";
import { DashboardPanel } from "./components/DashboardPanel";
import { SettingsPanel } from "./components/SettingsPanel";
import { ModelDetailPanel } from "./components/ModelDetailPanel";

// ─── App ───────────────────────────────────────────────────
function App() {
  const [view, setView] = React.useState<ViewName>("dashboard");
  const [model, setModel] = React.useState<ModelName>("flash");
  const [provider, setProviderState] = React.useState<Provider>("deepseek");
  const [balance, setBalance] = React.useState<BalanceData | MimoBalanceData | null>(null);
  const [balanceState, setBalanceState] = React.useState<BalanceState>("loading");
  const [balanceError, setBalanceError] = React.useState("");
  const [usage, setUsage] = React.useState<UsageResult | MimoUsageResult | null>(null);
  const [usageState, setUsageState] = React.useState<BalanceState>("loading");
  const [usageError, setUsageError] = React.useState("");
  const [refreshIntervalSeconds, setRefreshIntervalSeconds] = React.useState(60);
  const [autoRefreshEnabled, setAutoRefreshEnabled] = React.useState(false);

  const loadBalance = React.useCallback((p?: Provider) => {
    const active = p ?? provider;
    setBalanceState("loading");
    const cmd = active === "deepseek" ? "fetch_balance" : "fetch_mimo_balance";
    void fetchWithCache<BalanceData | MimoBalanceData>(`dsm-balance-${active}`, () => invoke<BalanceData | MimoBalanceData>(cmd))
      .then((data) => { setBalance(data); setBalanceState("ok"); })
      .catch((error) => {
        const message = typeof error === "string" ? error : "查询失败";
        setBalance(null); setBalanceError(message); setBalanceState(message.includes("未配置") ? "nokey" : "error");
      });
  }, [provider]);

  const loadUsage = React.useCallback((p?: Provider) => {
    const active = p ?? provider;
    setUsageState("loading");
    if (active === "deepseek") {
      void fetchWithCache<UsageResult>("dsm-usage-deepseek", () => invoke<UsageResult>("fetch_usage", { month: new Date().getMonth() + 1, year: new Date().getFullYear() }).then(async (current) => {
        const now = new Date();
        const needsPrev = addDays(now, -6).getMonth() !== now.getMonth();
        if (!needsPrev) return current;
        try {
          const prev = previousMonth(now);
          const prevUsage = await invoke<UsageResult>("fetch_usage", { month: prev.month, year: prev.year });
          return { ...current, days: [...prevUsage.days, ...current.days] };
        } catch { return current; }
      }))
        .then((data) => { setUsage(data); setUsageState("ok"); setUsageError(""); })
        .catch((error) => {
          const message = typeof error === "string" ? error : "查询失败"; setUsageError(message); setUsage(null); setUsageState(message.includes("未配置") ? "nokey" : "error");
        });
    } else {
      const now = new Date();
      void fetchWithCache<MimoUsageResult>("dsm-usage-mimo", () => invoke<MimoUsageResult>("fetch_mimo_usage", { month: now.getMonth() + 1, year: now.getFullYear() }))
        .then((data) => { setUsage(data); setUsageState("ok"); setUsageError(""); })
        .catch((error) => {
          const message = typeof error === "string" ? error : "查询失败"; setUsageError(message); setUsage(null); setUsageState(message.includes("未配置") ? "nokey" : "error");
        });
    }
  }, [provider]);

  const refreshAll = React.useCallback(() => { loadBalance(); loadUsage(); }, [loadBalance, loadUsage]);

  const setProvider = React.useCallback((next: Provider) => {
    setProviderState(next);
    setBalance(null); setBalanceState("loading");
    setUsage(null); setUsageState("loading");
    if (next === "mimo") void invoke("ensure_mimo_webview").catch(console.warn);
    void invoke<AppConfig>("set_provider", { provider: next }).catch(console.warn);
    // loadBalance/loadUsage 由 useEffect 监听 provider 变化后统一调用，避免双重调用竞态
  }, []);

  const providerRef = React.useRef(provider);
  const initialLoadDone = React.useRef(false);

  React.useEffect(() => {
    if (providerRef.current !== provider) { providerRef.current = provider; loadBalance(provider); loadUsage(provider); }
  }, [provider, loadBalance, loadUsage]);

  React.useEffect(() => {
    void invoke<AppConfig>("get_app_config")
      .then((config) => {
        if (!initialLoadDone.current) {
          initialLoadDone.current = true;
          if (config.provider !== providerRef.current) { setBalance(null); setBalanceState("loading"); setUsage(null); setUsageState("loading"); }
          providerRef.current = config.defaultProvider || config.provider; setProviderState(config.defaultProvider || config.provider);
          setRefreshIntervalSeconds(config.refreshIntervalSeconds || 60); setAutoRefreshEnabled(config.autoRefreshEnabled);
          loadBalance(config.provider); loadUsage(config.provider);
        }
      })
      .catch(() => { if (!initialLoadDone.current) { initialLoadDone.current = true; setRefreshIntervalSeconds(60); setAutoRefreshEnabled(false); loadBalance(); loadUsage(); } });
  }, [loadBalance, loadUsage]);

  React.useEffect(() => {
    if (!autoRefreshEnabled) return;
    const timer = window.setInterval(refreshAll, refreshIntervalSeconds * 1000);
    return () => window.clearInterval(timer);
  }, [autoRefreshEnabled, refreshAll, refreshIntervalSeconds]);

  React.useEffect(() => {
    const unlistenPromise = listen("mimo-auth-required", () => {
      setUsageState("error"); setUsageError("MiMo 未登录，请在设置中重新登录小米账号");
      setBalanceState("error"); setBalanceError("MiMo 未登录");
    });
    return () => { void unlistenPromise.then((unlisten) => unlisten()); };
  }, []);

  const hideWindow = React.useCallback(() => { void invoke("hide_main_window").catch(() => {}); }, []);

  return (
    <div className="stage">
      {view === "dashboard" && (
        <DashboardPanel
          provider={provider} onProviderChange={setProvider}
          balance={balance} balanceState={balanceState} balanceError={balanceError}
          usage={usage} usageState={usageState} usageError={usageError}
          onRefresh={refreshAll} onClose={hideWindow}
          onSettings={() => setView("settings")}
          onDetail={(nextModel) => { setModel(nextModel); setView("detail"); }}
        />
      )}
      {view === "settings" && (
        <SettingsPanel
          provider={provider} onProviderChange={setProvider} onBack={() => setView("dashboard")}
          onUsageLoaded={(nextUsage) => { setUsage(nextUsage); setUsageState("ok"); }}
          onUsageCleared={() => { setUsage(null); setUsageState("loading"); }}
          onRefreshIntervalChanged={setRefreshIntervalSeconds} onAutoRefreshChanged={setAutoRefreshEnabled}
        />
      )}
      {view === "detail" && (
        <ModelDetailPanel model={model} usage={usage} usageState={usageState} onBack={() => setView("dashboard")} provider={provider} />
      )}
    </div>
  );
}


// ─── Mount ─────────────────────────────────────────────────
// Apply the saved theme before first render to avoid a flash of the wrong skin.
document.documentElement.setAttribute("data-theme", localStorage.getItem("ui-theme") || "light");

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
