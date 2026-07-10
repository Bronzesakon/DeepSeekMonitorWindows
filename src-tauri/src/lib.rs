//! DeepSeek / MiMo Monitor Windows — Tauri 入口
//!
//! 职责：模块声明、Tauri 命令注册、Builder 配置。
//! 具体业务逻辑分散在 modules/ 子模块中。

mod modules;
use modules::{
    config, deepseek, mimo, tray,
    types::{
        AppConfig, BalanceResult, CallbackServerPort, MimoBalanceResult, MimoDetailCache,
        MimoUsageResult, UsageResult,
    },
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;

use tauri::{Manager, WebviewWindow};

// ─── Callback Server（持久化 tiny_http）───────────────────

struct CallbackServer {
    port: u16,
}

impl CallbackServer {
    fn start(shared_map: Arc<Mutex<HashMap<String, oneshot::Sender<String>>>>) -> std::io::Result<Self> {
        use tiny_http::{Header, Method, Response, Server};
        let server = Server::http("127.0.0.1:0").map_err(|e| std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, format!("无法启动回调服务器：{e}")))?;
        let port = server.server_addr().to_ip().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "回调服务器地址无效"))?.port();
        std::thread::spawn(move || {
            while let Ok(Some(mut request)) =
                server.recv_timeout(std::time::Duration::from_secs(3600))
            {
                if *request.method() == Method::Options {
                    let response = Response::from_string(String::new())
                        .with_header(
                            Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"null"[..])
                                .unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(
                                &b"Access-Control-Allow-Methods"[..],
                                &b"POST, OPTIONS"[..],
                            )
                            .unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(
                                &b"Access-Control-Allow-Headers"[..],
                                &b"Content-Type"[..],
                            )
                            .unwrap(),
                        );
                    let _ = request.respond(response);
                } else {
                    let mut body = String::new();
                    let _ = request.as_reader().read_to_string(&mut body);
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&body) {
                        if let (Some(rid), Some(data)) = (
                            parsed.get("reqId").and_then(|v| v.as_str()),
                            parsed.get("data").and_then(|v| v.as_str()),
                        ) {
                            let mut map = shared_map.lock().unwrap();
                            if let Some(tx) = map.remove(rid) {
                                let _ = tx.send(data.to_string());
                            }
                        }
                    }
                    let response = Response::from_string("OK").with_header(
                        Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"null"[..]).unwrap(),
                    );
                    let _ = request.respond(response);
                }
            }
        });
        Ok(CallbackServer { port })
    }
}

// ─── Tauri 命令 ──────────────────────────────────────────

#[tauri::command]
fn hide_main_window(window: WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|error| error.to_string())
}

#[tauri::command]
fn update_tray_tooltip(app: tauri::AppHandle, text: String) -> Result<(), String> {
    let state = app.state::<tray::TrayStateInner>();
    let tray = state.0.lock().map_err(|e| e.to_string())?;
    tray.set_tooltip(Some(&text)).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_app_config() -> Result<AppConfig, String> {
    config::to_app_config(config::read_stored_config()?)
}

#[tauri::command]
fn save_api_key(api_key: String) -> Result<AppConfig, String> {
    let value = api_key.trim().to_string();
    if value.is_empty() {
        return Err("API Key 不能为空".to_string());
    }
    if value.len() > 256 {
        return Err("API Key 长度超出限制".to_string());
    }
    if !value.starts_with("sk-") {
        log::warn!("API Key 格式异常：不以 sk- 开头");
    }
    let mut config = config::read_stored_config()?;
    config.api_key = Some(value);
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn clear_api_key() -> Result<AppConfig, String> {
    let mut config = config::read_stored_config()?;
    config.api_key = None;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_refresh_interval(refresh_interval_seconds: u64) -> Result<AppConfig, String> {
    let mut config = config::read_stored_config()?;
    config.refresh_interval_seconds = refresh_interval_seconds;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_auto_refresh_enabled(auto_refresh_enabled: bool) -> Result<AppConfig, String> {
    let mut config = config::read_stored_config()?;
    config.auto_refresh_enabled = auto_refresh_enabled;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_autostart(autostart: bool) -> Result<AppConfig, String> {
    config::apply_autostart(autostart)?;
    let mut config = config::read_stored_config()?;
    config.autostart = autostart;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_low_balance_notify(enabled: bool) -> Result<AppConfig, String> {
    let mut config = config::read_stored_config()?;
    config.low_balance_notify = enabled;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_low_balance_threshold(threshold: f64) -> Result<AppConfig, String> {
    if !threshold.is_finite() || threshold < 0.0 {
        return Err("阈值必须为非负数".to_string());
    }
    let mut config = config::read_stored_config()?;
    config.low_balance_threshold = threshold;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_theme(theme: String) -> Result<AppConfig, String> {
    if !["light", "dark", "system"].contains(&theme.as_str()) {
        return Err("无效主题".to_string());
    }
    let mut config = config::read_stored_config()?;
    config.theme = theme;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_default_provider(provider: String) -> Result<AppConfig, String> {
    if !["deepseek", "mimo"].contains(&provider.as_str()) {
        return Err("无效平台".to_string());
    }
    let mut config = config::read_stored_config()?;
    config.default_provider = provider;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_mimo_refresh_interval(seconds: u64) -> Result<AppConfig, String> {
    let mut config = config::read_stored_config()?;
    config.mimo_refresh_interval_seconds = seconds;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
fn save_notify_cooldown(minutes: u64) -> Result<AppConfig, String> {
    let mut config = config::read_stored_config()?;
    config.notify_cooldown_minutes = minutes;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

/// 余额检查并发送 Windows 通知
fn check_and_notify_low_balance(_app: &tauri::AppHandle, balance: &BalanceResult) {
    let config = match config::read_stored_config() {
        Ok(c) => c,
        Err(_) => return,
    };
    if !config.low_balance_notify {
        return;
    }
    let threshold = config.low_balance_threshold;
    if threshold <= 0.0 {
        return;
    }
    let balance_val = match balance.total_balance.parse::<f64>() {
        Ok(v) => v,
        Err(_) => return,
    };
    if balance_val < threshold {
        let _ = notify_rust::Notification::new()
            .summary("DeepSeek / MiMo Monitor")
            .body(&format!("余额不足提醒：当前余额 ¥{}，低于阈值 ¥{}", balance.total_balance, threshold))
            .appname("DeepSeekMonitor")
            .show();
        log::info!("[Notify] 余额不足: ¥{} < ¥{}", balance.total_balance, threshold);
    }
}

#[tauri::command]
fn set_provider(provider: String) -> Result<AppConfig, String> {
    if provider != "deepseek" && provider != "mimo" {
        return Err("无效的 provider，仅支持 deepseek 或 mimo".to_string());
    }
    let mut config = config::read_stored_config()?;
    config.provider = provider;
    config::write_stored_config(&config)?;
    config::to_app_config(config)
}

#[tauri::command]
async fn fetch_balance(app: tauri::AppHandle) -> Result<BalanceResult, String> {
    let result = deepseek::do_fetch_balance().await?;
    check_and_notify_low_balance(&app, &result);
    Ok(result)
}

#[tauri::command]
fn save_usage_token(usage_token: String) -> Result<AppConfig, String> {
    let value = usage_token.trim().to_string();
    if value.is_empty() {
        return Err("用量 Token 不能为空".to_string());
    }
    if value.len() > 4096 {
        return Err("用量 Token 长度超出限制".to_string());
    }
    deepseek::do_save_usage_token(usage_token)
}

#[tauri::command]
fn clear_usage_token() -> Result<AppConfig, String> {
    deepseek::do_clear_usage_token()
}

#[tauri::command]
async fn start_usage_sync(app: tauri::AppHandle) -> Result<bool, String> {
    deepseek::start_usage_sync(&app)
}

#[tauri::command]
async fn usage_token_captured(
    app: tauri::AppHandle,
    token: String,
    month: u32,
    year: u32,
) -> Result<AppConfig, String> {
    deepseek::do_usage_token_captured(&app, token, month, year).await
}

#[tauri::command]
async fn fetch_usage(month: u32, year: u32) -> Result<UsageResult, String> {
    deepseek::do_fetch_usage(month, year).await
}

#[tauri::command]
async fn fetch_mimo_balance(app: tauri::AppHandle) -> Result<MimoBalanceResult, String> {
    let result = mimo::do_fetch_mimo_balance(&app).await?;
    // 检查 MiMo 余额是否低于阈值
    let config = config::read_stored_config().unwrap_or_default();
    if config.low_balance_notify && config.low_balance_threshold > 0.0 {
        if let Ok(val) = result.available_balance.parse::<f64>() {
            if val < config.low_balance_threshold {
                let _ = notify_rust::Notification::new()
                    .summary("DeepSeek / MiMo Monitor")
                    .body(&format!("余额不足提醒：当前余额 ¥{}，低于阈值 ¥{}", result.available_balance, config.low_balance_threshold))
                    .appname("DeepSeekMonitor")
                    .show();
                log::info!("[Notify] MiMo 余额不足: ¥{} < ¥{}", result.available_balance, config.low_balance_threshold);
            }
        }
    }
    Ok(result)
}

#[tauri::command]
async fn fetch_mimo_usage(
    app: tauri::AppHandle,
    month: u32,
    year: u32,
) -> Result<MimoUsageResult, String> {
    mimo::do_fetch_mimo_usage(&app, month, year).await
}

#[tauri::command]
async fn start_mimo_sync(app: tauri::AppHandle) -> Result<bool, String> {
    mimo::do_start_mimo_sync(&app)
}

#[tauri::command]
async fn ensure_mimo_webview(app: tauri::AppHandle) -> Result<(), String> {
    mimo::do_ensure_mimo_webview(&app)
}

#[tauri::command]
fn mimo_api_response(
    app: tauri::AppHandle,
    req_id: String,
    json: String,
) -> Result<(), String> {
    mimo::do_mimo_api_response(&app, req_id, json)
}

// ─── 自动更新 ──────────────────────────────────────────────

struct PendingUpdate(std::sync::Mutex<Option<tauri_plugin_updater::Update>>);

#[derive(serde::Serialize)]
struct UpdateInfo {
    version: String,
    date: String,
    body: String,
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "event", content = "data")]
enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started { content_length: Option<u64> },
    #[serde(rename_all = "camelCase")]
    Progress { chunk_length: usize, downloaded: u64 },
    Finished,
}

#[tauri::command]
async fn check_update(app: tauri::AppHandle, pending: tauri::State<'_, PendingUpdate>) -> Result<Option<UpdateInfo>, String> {
    use tauri_plugin_updater::UpdaterExt;
    let updater = app.updater().map_err(|e| format!("获取更新器失败：{e}"))?;
    match updater.check().await {
        Ok(Some(update)) => {
            let info = UpdateInfo {
                version: update.version.clone(),
                date: update.date.map(|d| {
                    let y = d.year();
                    let m = d.month() as u8;
                    let day = d.day();
                    format!("{y}-{m:02}-{day:02}")
                }).unwrap_or_default(),
                body: update.body.clone().unwrap_or_default(),
            };
            *pending.0.lock().unwrap() = Some(update);
            Ok(Some(info))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(format!("检查更新失败：{e}")),
    }
}

#[tauri::command]
async fn install_update(pending: tauri::State<'_, PendingUpdate>, on_event: tauri::ipc::Channel<DownloadEvent>) -> Result<(), String> {
    let update = pending.0.lock().unwrap().take().ok_or("没有待安装的更新")?;
    let mut downloaded: u64 = 0;
    let mut started = false;
    update
        .download_and_install(
            |chunk_len, content_len| {
                if !started {
                    let _ = on_event.send(DownloadEvent::Started { content_length: content_len });
                    started = true;
                }
                downloaded += chunk_len as u64;
                let _ = on_event.send(DownloadEvent::Progress { chunk_length: chunk_len, downloaded });
            },
            || {
                let _ = on_event.send(DownloadEvent::Finished);
            },
        )
        .await
        .map_err(|e| format!("下载安装失败：{e}"))?;
    Ok(())
}

// ─── 主入口 ──────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                tray::show_main_window(&window);
            }
        }))
        .manage(Arc::new(std::sync::atomic::AtomicBool::new(false)))
        .manage(Arc::new(Mutex::new(HashMap::<String, oneshot::Sender<String>>::new())))
        .manage(Arc::new(tokio::sync::Mutex::new(())))
        .manage(Mutex::new(MimoDetailCache::new()))
        .manage(Mutex::new(CallbackServerPort(0)))
        .manage(PendingUpdate(std::sync::Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            hide_main_window,
            update_tray_tooltip,
            get_app_config,
            save_api_key,
            clear_api_key,
            save_refresh_interval,
            save_auto_refresh_enabled,
            save_autostart,
            save_low_balance_notify,
            save_low_balance_threshold,
            save_theme,
            save_default_provider,
            save_mimo_refresh_interval,
            save_notify_cooldown,
            set_provider,
            fetch_balance,
            save_usage_token,
            clear_usage_token,
            fetch_usage,
            start_usage_sync,
            usage_token_captured,
            fetch_mimo_balance,
            fetch_mimo_usage,
            start_mimo_sync,
            ensure_mimo_webview,
            mimo_api_response,
            check_update,
            install_update
        ])
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 启动持久化回调服务器
            let shared_map = app
                .state::<Arc<Mutex<HashMap<String, oneshot::Sender<String>>>>>()
                .inner()
                .clone();
            let cb_server = CallbackServer::start(shared_map)?;
            *app.state::<Mutex<CallbackServerPort>>().lock().unwrap() =
                CallbackServerPort(cb_server.port);
            app.manage(Mutex::new(cb_server));

            // 初始化托盘
            tray::setup_tray(app)?;

            // 首次启动定位到右下角（固定尺寸窗口）
            if let Some(window) = app.get_webview_window("main") {
                let _ = tray::position_near_tray(&window);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
