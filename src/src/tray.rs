use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};
use std::sync::Arc;

pub fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let storage = app.state::<Arc<crate::storage::Storage>>();

    // 加载 DeepSeek cookies
    let ds_state = app.state::<crate::ds::api::ApiState>();
    {
        if let Some(t) = storage.load_platform_token() {
            *ds_state.platform_token.lock().unwrap_or_else(|e| e.into_inner()) = Some(t);
        }
        if let Some(c) = storage.load_platform_cookies() {
            *ds_state.platform_cookies.lock().unwrap_or_else(|e| e.into_inner()) = Some(c);
        }
    }

    // 加载 MiMo cookies
    let mimo_state = app.state::<crate::mimo::api::ApiState>();
    {
        if let Some(c) = storage.load_mimo_platform_cookies() {
            *mimo_state.platform_cookies.lock().unwrap_or_else(|e| e.into_inner()) = Some(c);
        }
    }

    // 构建托盘菜单
    let show_panel = CheckMenuItemBuilder::with_id("show_panel", "显示面板")
        .checked(true)
        .build(app)?;
    let toggle_widget = CheckMenuItemBuilder::with_id("toggle_widget", "显示小组件")
        .checked(false)
        .build(app)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let refresh = MenuItemBuilder::with_id("refresh", "刷新").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&show_panel, &toggle_widget, &sep1, &refresh, &settings, &sep2, &quit])
        .build()?;

    let mut tray_builder = TrayIconBuilder::with_id("tray");
    if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
    }
    let tray = tray_builder
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "show_panel" => {
                    // Copy widget position to main before closing widget
                    if let Some(w) = app.get_webview_window("widget") {
                        if let Ok(pos) = w.outer_position() {
                            crate::windows::save_window_pos_raw(app, "main", pos.x, pos.y);
                        }
                    }
                    if app.get_webview_window("widget").map(|w| w.is_visible().unwrap_or(false)).unwrap_or(false) {
                        crate::windows::toggle_widget(app);
                    }
                    crate::windows::show_panel(app);
                }
                "toggle_widget" => {
                    // Copy main position to widget before hiding main
                    if let Some(w) = app.get_webview_window("main") {
                        if let Ok(pos) = w.outer_position() {
                            crate::windows::save_window_pos_raw(app, "widget", pos.x, pos.y);
                        }
                    }
                    if let Some(w) = app.get_webview_window("main") {
                        let _ = w.hide();
                    }
                    crate::windows::windows_hidden(app, "main");
                    crate::windows::toggle_widget(app);
                }
                "refresh" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = app_handle.emit("ds-trigger-refresh", ());
                        let _ = app_handle.emit("mimo-trigger-refresh", ());
                    });
                }
                "settings" => crate::windows::show_settings(app),
                "quit" => {
                    crate::windows::save_window_pos(app, "main");
                    crate::windows::save_window_pos(app, "widget");
                    crate::windows::save_window_pos(app, "settings");
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                tauri::tray::TrayIconEvent::Click {
                    button: tauri::tray::MouseButton::Left,
                    button_state: tauri::tray::MouseButtonState::Down,
                    ..
                } => {
                    crate::windows::toggle_panel(tray.app_handle());
                }
                _ => {}
            }
        })
        .build(app)?;

    Box::leak(Box::new(tray));
    app.manage(Arc::new(menu));

    Ok(())
}

// ─── Shared Commands ─────────────────────────────────────────

#[tauri::command]
pub fn save_setting(
    storage: tauri::State<'_, Arc<crate::storage::Storage>>,
    notifier: tauri::State<'_, Arc<crate::RefreshNotifier>>,
    key: String,
    value: String,
) -> Result<(), String> {
    storage.save_setting(&key, &value);
    if key == "refresh_interval" {
        if let Ok(secs) = value.parse::<u64>() {
            let _ = notifier.tx.send(secs);
            crate::debug_log!("[save_setting] 刷新间隔变更为 {} 秒", secs);
        }
    }
    Ok(())
}

#[tauri::command]
pub fn load_setting(
    storage: tauri::State<'_, Arc<crate::storage::Storage>>,
    key: String,
) -> Result<Option<String>, String> {
    Ok(storage.load_setting(&key))
}

#[tauri::command]
pub async fn show_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    crate::windows::show_settings(&app);
    Ok(())
}

#[tauri::command]
pub fn is_windows11() -> bool {
    #[cfg(target_os = "windows")]
    {
        #[repr(C)]
        struct OsVersionInfoExW {
            dw_os_version_info_size: u32,
            _major: u32,
            _minor: u32,
            dw_build_number: u32,
            _platform_id: u32,
            _csd_version: [u16; 128],
            _service_pack_major: u16,
            _service_pack_minor: u16,
            _suite_mask: u16,
            _product_type: u8,
            _reserved: u8,
        }
        extern "system" {
            fn RtlGetVersion(info: *mut OsVersionInfoExW) -> i32;
        }
        unsafe {
            let mut info: OsVersionInfoExW = std::mem::zeroed();
            info.dw_os_version_info_size = std::mem::size_of::<OsVersionInfoExW>() as u32;
            if RtlGetVersion(&mut info) == 0 {
                return info.dw_build_number >= 22000;
            }
        }
        false
    }
    #[cfg(not(target_os = "windows"))]
    false
}

#[tauri::command]
pub async fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
    crate::windows::close_panel(&app);
    Ok(())
}

#[tauri::command]
pub async fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(main_win) = app.get_webview_window("main") {
        main_win.show().map_err(|e| e.to_string())?;
        main_win.set_focus().map_err(|e| e.to_string())?;
        crate::windows::windows_shown(&app, "main");
        crate::windows::update_tray_menu_checks(&app);
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    crate::windows::hide_settings_window(&app);
    Ok(())
}

#[tauri::command]
pub async fn recenter_settings(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("settings") {
        let _ = win.center();
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_widget_window(app: tauri::AppHandle) -> Result<(), String> {
    crate::windows::save_window_pos(&app, "widget");
    if let Some(win) = app.get_webview_window("widget") {
        let _ = win.hide();
    }
    crate::windows::windows_hidden(&app, "widget");
    crate::windows::update_tray_menu_checks(&app);
    Ok(())
}

#[tauri::command]
pub fn save_window_position(
    storage: tauri::State<'_, Arc<crate::storage::Storage>>,
    window: String,
    x: i32,
    y: i32,
) -> Result<(), String> {
    storage.save_setting(&format!("{}_x", window), &x.to_string());
    storage.save_setting(&format!("{}_y", window), &y.to_string());
    Ok(())
}

#[tauri::command]
pub fn load_window_position(
    storage: tauri::State<'_, Arc<crate::storage::Storage>>,
    window: String,
) -> Result<Option<(i32, i32)>, String> {
    let x = storage.load_setting(&format!("{}_x", window)).and_then(|v| v.parse().ok());
    let y = storage.load_setting(&format!("{}_y", window)).and_then(|v| v.parse().ok());
    match (x, y) {
        (Some(x), Some(y)) => Ok(Some((x, y))),
        _ => Ok(None),
    }
}

#[tauri::command]
pub async fn reposition_trend_detail(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(detail_win) = app.get_webview_window("trend-detail") {
        if detail_win.is_visible().unwrap_or(false) {
            crate::windows::reposition_trend_detail(&app, &detail_win);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn broadcast_edge_snap_setting(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let _ = app.emit("edge-snap-setting-changed", enabled);
    Ok(())
}

#[tauri::command]
pub async fn get_auto_start() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};
        let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
        Ok(hkcu.open_subkey_with_flags(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run", KEY_READ)
            .ok()
            .and_then(|key| key.get_value::<String, _>("DeepSeekDesktopAssistant").ok())
            .is_some())
    }
    #[cfg(not(target_os = "windows"))]
    Ok(false)
}

#[tauri::command]
pub async fn set_auto_start(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
        let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu.open_subkey_with_flags(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run", KEY_SET_VALUE)
            .map_err(|e| e.to_string())?;
        if enabled {
            let exe_path = std::env::current_exe().map_err(|e| e.to_string())?.to_string_lossy().to_string();
            key.set_value("DeepSeekDesktopAssistant", &exe_path).map_err(|e| e.to_string())
        } else {
            match key.delete_value("DeepSeekDesktopAssistant") {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(())
}

#[tauri::command]
pub async fn show_trend_detail_window(
    state: tauri::State<'_, crate::TrendDetailState>,
    app: tauri::AppHandle,
    date: Option<String>,
    center_x: Option<f64>,
    center_y: Option<f64>,
    _parent_width: Option<f64>,
    _parent_height: Option<f64>,
) -> Result<(), String> {
    if let Some(ref date) = date {
        let data = serde_json::json!({
            "date": date,
            "center_x": center_x,
            "center_y": center_y,
        });
        if let Ok(mut lock) = state.data.lock() {
            *lock = data;
        }
        crate::windows::show_trend_detail(&app, date, 0, 0, 0, "0%", 0).await;
    }
    Ok(())
}

#[tauri::command]
pub fn is_window_docked(window: tauri::WebviewWindow) -> Result<bool, String> {
    Ok(window.is_visible().unwrap_or(false) && window.is_always_on_top().unwrap_or(false))
}

#[tauri::command]
pub fn snap_to_edge(window: tauri::WebviewWindow) -> Result<(), String> {
    let _ = window.set_always_on_top(true);
    Ok(())
}

#[tauri::command]
pub fn set_dragging(_window: tauri::WebviewWindow, _dragging: bool) -> Result<(), String> {
    crate::debug_log!("[set_dragging] label={}, dragging={}", _window.label(), _dragging);
    Ok(())
}

#[tauri::command]
pub fn get_monitor_top_y(window: tauri::WebviewWindow) -> Result<i32, String> {
    window.current_monitor()
        .ok()
        .flatten()
        .map(|m| (m.position().y as f64 / m.scale_factor()) as i32)
        .ok_or("no monitor".to_string())
}
