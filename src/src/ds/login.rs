use crate::storage::Storage;
use std::sync::Arc;
use tauri::{Emitter, Manager, WebviewWindowBuilder};

pub async fn start_login_flow(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("login") {
        let _ = win.close();
    }

    let api_state = app.state::<crate::ds::api::ApiState>();
    let already_logged_in = api_state
        .platform_token
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .is_some();

    let app_for_poll = app.clone();
    let app_for_nav = app.clone();
    let app_for_watch = app.clone();

    let builder = WebviewWindowBuilder::new(
        &app,
        "login",
        tauri::WebviewUrl::External("https://platform.deepseek.com/usage".parse().unwrap()),
    )
    .title("DeepSeek Platform 登录")
    .inner_size(1180.0, 860.0)
    .center()
    .always_on_top(true);

    let builder = if !already_logged_in {
        builder.initialization_script("localStorage.removeItem('userToken');")
    } else {
        builder
    };

    let login_window = builder
        .on_navigation(move |url| {
            let url_str = url.to_string();
            if url_str.starts_with("https://dsm.local/token") {
                let mut token = String::new();
                let mut cookies = String::new();

                if let Some(query) = url_str.split('?').nth(1) {
                    for pair in query.split('&') {
                        let mut parts = pair.splitn(2, '=');
                        let key = parts.next().unwrap_or("");
                        let val = parts.next().unwrap_or("");
                        match key {
                            "t" => token = urlencoding::decode(val).unwrap_or_default().into_owned(),
                            "c" => cookies = urlencoding::decode(val).unwrap_or_default().into_owned(),
                            _ => {}
                        }
                    }
                }

                if !token.is_empty() {
                    let api_state = app_for_nav.state::<crate::ds::api::ApiState>();
                    *api_state.platform_token.lock().unwrap_or_else(|e| e.into_inner()) =
                        Some(token.clone());
                    if !cookies.is_empty() {
                        *api_state.platform_cookies.lock().unwrap_or_else(|e| e.into_inner()) =
                            Some(cookies.clone());
                    }

                    let storage = app_for_nav.state::<Arc<Storage>>();
                    storage.save_platform_token(&token).ok();
                    if !cookies.is_empty() {
                        storage.save_platform_cookies(&cookies).ok();
                    }
                    storage.save_onboarding_completed();

                    if let Some(win) = app_for_nav.get_webview_window("login") {
                        let _ = win.close();
                    }
                    let _ = app_for_nav.emit("ds-login-complete", ());
                }

                false
            } else {
                true
            }
        })
        .build()
        .map_err(|e| e.to_string())?;

    // Inject JS polling script after 3 seconds
    let inject_js = r#"
        (function() {
            var checkInterval = setInterval(function() {
                var ut = localStorage.getItem('userToken');
                if (ut) {
                    try {
                        var o = JSON.parse(ut);
                        if (o && o.value) {
                            clearInterval(checkInterval);
                            var token = encodeURIComponent(o.value);
                            var cookies = encodeURIComponent(document.cookie);
                            window.location.href = 'https://dsm.local/token?t=' + token + '&c=' + cookies;
                        }
                    } catch(e) {}
                }
            }, 2000);
        })();
    "#;

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        if let Some(win) = app_for_poll.get_webview_window("login") {
            let _ = win.eval(inject_js);
        }
    });

    // Detect login window close via event (replaces polling)
    login_window.on_window_event(move |event| {
        if let tauri::WindowEvent::Destroyed = event {
            let api_state = app_for_watch.state::<crate::ds::api::ApiState>();
            if !api_state.has_platform_session() {
                let _ = app_for_watch.emit("ds-login-cancelled", ());
            }
        }
    });

    Ok(())
}
