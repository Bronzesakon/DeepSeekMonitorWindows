use crate::storage::Storage;
use std::sync::Arc;
use tauri::{Emitter, Manager, WebviewWindowBuilder};

pub async fn start_login_flow(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("login") {
        let _ = win.close();
    }

    let app_for_watch = app.clone();
    let app_for_poll = app.clone();

    let login_window = WebviewWindowBuilder::new(
        &app,
        "login",
        tauri::WebviewUrl::External("https://platform.xiaomimimo.com/".parse().unwrap()),
    )
    .title("MiMo Platform 登录")
    .inner_size(1180.0, 860.0)
    .center()
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;

    tokio::spawn(async move {
        for _ in 0..120 {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let win = match app_for_poll.get_webview_window("login") {
                Some(w) => w,
                None => return,
            };

            if !win.is_visible().unwrap_or(false) {
                return;
            }

            let cookies = match extract_cookies(&app_for_poll) {
                Ok(c) if !c.is_empty() => c,
                _ => continue,
            };

            let api_state = app_for_poll.state::<crate::mimo::api::ApiState>();
            {
                let mut guard = api_state
                    .platform_cookies
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                *guard = Some(cookies.clone());
            }

            let balance_res = crate::mimo::api::fetch_balance(Some(&cookies)).await;
            match balance_res {
                Ok(_) => {
                    let storage = app_for_poll.state::<Arc<Storage>>();
                    storage.save_mimo_platform_cookies(&cookies).ok();
                    storage.save_onboarding_completed();

                    if let Some(win) = app_for_poll.get_webview_window("login") {
                        let _ = win.close();
                    }
                    let _ = app_for_poll.emit("mimo-login-complete", ());
                    return;
                }
                Err(ref e) if e.is_auth_error() => {
                    let mut guard = api_state
                        .platform_cookies
                        .lock()
                        .unwrap_or_else(|e| e.into_inner());
                    *guard = None;
                    continue;
                }
                Err(_) => {
                    continue;
                }
            }
        }
    });

    login_window.on_window_event(move |event| {
        if let tauri::WindowEvent::Destroyed = event {
            let api_state = app_for_watch.state::<crate::mimo::api::ApiState>();
            if !api_state.has_platform_session() {
                let _ = app_for_watch.emit("mimo-login-cancelled", ());
            }
        }
    });

    Ok(())
}

#[cfg(target_os = "windows")]
fn extract_cookies(app: &tauri::AppHandle) -> Result<String, String> {
    use std::sync::mpsc;
    use webview2_com::GetCookiesCompletedHandler;
    use webview2_com::Microsoft::Web::WebView2::Win32::*;
    use windows_core::{Interface, PWSTR};

    let win = app
        .get_webview_window("login")
        .ok_or("login window not found")?;

    let (tx, rx) = mpsc::channel::<Result<String, String>>();

    let tx_for_err = tx.clone();
    let result = win.with_webview(move |platform_webview| unsafe {
        let controller = platform_webview.controller();

        let webview = match controller.CoreWebView2() {
            Ok(wv) => wv,
            Err(e) => {
                let _ = tx.send(Err(format!("get webview failed: {}", e)));
                return;
            }
        };

        let webview2: ICoreWebView2_2 = match webview.cast() {
            Ok(wv2) => wv2,
            Err(e) => {
                let _ = tx.send(Err(format!("cast failed: {}", e)));
                return;
            }
        };

        let cookie_mgr = match webview2.CookieManager() {
            Ok(mgr) => mgr,
            Err(e) => {
                let _ = tx.send(Err(format!("CookieManager failed: {}", e)));
                return;
            }
        };

        let uri = windows_core::HSTRING::from("https://platform.xiaomimimo.com");

        let handler = GetCookiesCompletedHandler::create(Box::new(
            move |error_code: windows_core::Result<()>,
                  cookie_list: Option<ICoreWebView2CookieList>|
                  -> windows_core::Result<()> {
                if let Err(e) = error_code {
                    let _ = tx.send(Err(format!("GetCookies callback error: {}", e)));
                    return Ok(());
                }

                let cookie_list = match cookie_list {
                    Some(list) => list,
                    None => {
                        let _ = tx.send(Err("cookie_list is null".to_string()));
                        return Ok(());
                    }
                };

                let mut count: u32 = 0;
                if let Err(e) = cookie_list.Count(&mut count) {
                    let _ = tx.send(Err(format!("Count failed: {}", e)));
                    return Ok(());
                }

                let mut parts = Vec::new();

                for i in 0..count {
                    let cookie = match cookie_list.GetValueAtIndex(i) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };

                    let mut name_pwstr = PWSTR::null();
                    let mut value_pwstr = PWSTR::null();

                    if cookie.Name(&mut name_pwstr).is_err() {
                        continue;
                    }
                    if cookie.Value(&mut value_pwstr).is_err() {
                        windows::Win32::System::Com::CoTaskMemFree(
                            Some(name_pwstr.as_ptr() as *const _),
                        );
                        continue;
                    }

                    let name = name_pwstr.to_string().unwrap_or_default();
                    let value = value_pwstr.to_string().unwrap_or_default();

                    windows::Win32::System::Com::CoTaskMemFree(
                        Some(name_pwstr.as_ptr() as *const _),
                    );
                    windows::Win32::System::Com::CoTaskMemFree(
                        Some(value_pwstr.as_ptr() as *const _),
                    );

                    if !name.is_empty() {
                        let needs_quoting = value.contains('=')
                            || value.contains(',')
                            || value.contains(';')
                            || value.contains(' ')
                            || value.contains('"');
                        let formatted_value = if needs_quoting
                            && !value.starts_with('"')
                            && !value.ends_with('"')
                        {
                            format!("\"{}\"", value.replace('"', "\\\""))
                        } else {
                            value
                        };
                        parts.push(format!("{}={}", name, formatted_value));
                    }
                }

                let _ = tx.send(Ok(parts.join("; ")));
                Ok(())
            },
        ));

        if let Err(e) = cookie_mgr.GetCookies(&uri, &handler) {
            let _ = tx_for_err.send(Err(format!("GetCookies call failed: {}", e)));
        }
    });

    if let Err(e) = result {
        return Err(format!("with_webview failed: {}", e));
    }

    rx.recv_timeout(std::time::Duration::from_secs(5))
        .map_err(|_| "cookie extraction timed out".to_string())?
}

/// 清理 WebView2 存储的所有 cookie
/// WebView2 所有窗口共享同一个用户数据目录, 从任意存在的 webview 获取 CookieManager 即可
/// 返回是否存在可用的 webview (用于前端提示)
#[cfg(target_os = "windows")]
pub fn clear_webview_cookies(app: &tauri::AppHandle) -> bool {
    use webview2_com::Microsoft::Web::WebView2::Win32::*;
    use windows_core::Interface;

    // 优先用 login 窗口, 其次 main 窗口
    let win = app
        .get_webview_window("login")
        .or_else(|| app.get_webview_window("main"));

    let Some(win) = win else {
        crate::debug_log!("[clear_webview_cookies] 没有可用的 webview 窗口");
        return false;
    };

    let result = win.with_webview(move |platform_webview| unsafe {
        let controller = platform_webview.controller();
        if let Ok(webview) = controller.CoreWebView2() {
            if let Ok(webview2) = webview.cast::<ICoreWebView2_2>() {
                if let Ok(cookie_mgr) = webview2.CookieManager() {
                    if let Err(_e) = cookie_mgr.DeleteAllCookies() {
                        crate::debug_log!("[clear_webview_cookies] DeleteAllCookies 失败: {}", _e);
                    } else {
                        crate::debug_log!("[clear_webview_cookies] DeleteAllCookies 成功");
                    }
                }
            }
        }
    });

    if let Err(_e) = result {
        crate::debug_log!("[clear_webview_cookies] with_webview 失败: {}", _e);
        return false;
    }

    true
}

#[cfg(not(target_os = "windows"))]
pub fn clear_webview_cookies(_app: &tauri::AppHandle) -> bool {
    false
}

#[cfg(not(target_os = "windows"))]
fn extract_cookies(_app: &tauri::AppHandle) -> Result<String, String> {
    Err("cookie extraction only supported on Windows".to_string())
}
