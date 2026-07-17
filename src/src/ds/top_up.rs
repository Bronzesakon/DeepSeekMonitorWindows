use tauri::{Manager, WebviewWindowBuilder};

pub async fn open_top_up_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("top-up") {
        let _ = win.close();
    }

    WebviewWindowBuilder::new(
        &app,
        "top-up",
        tauri::WebviewUrl::External(
            "https://platform.deepseek.com/top_up".parse().unwrap(),
        ),
    )
    .title("DeepSeek 充值")
    .inner_size(1180.0, 860.0)
    .center()
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}