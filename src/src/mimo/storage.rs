use serde_json;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub struct Storage {
    data_dir: OnceLock<PathBuf>,
    write_lock: Mutex<()>,
}

impl Storage {
    pub fn new() -> Self {
        Self { data_dir: OnceLock::new(), write_lock: Mutex::new(()) }
    }

    pub fn init(&self, data_dir: PathBuf) {
        fs::create_dir_all(&data_dir).ok();
        self.data_dir.set(data_dir).ok();
    }

    pub fn data_dir(&self) -> &PathBuf {
        self.data_dir.get().expect("Storage not initialized")
    }

    fn settings_path(&self) -> PathBuf {
        self.data_dir().join("settings.json")
    }

    // MARK: - Refresh Interval

    pub fn load_refresh_interval(&self) -> f64 {
        self.load_setting("refresh_interval")
            .and_then(|v| v.parse().ok())
            .unwrap_or(60.0)
    }

    pub fn save_refresh_interval(&self, interval: f64) {
        self.save_setting("refresh_interval", &interval.to_string());
    }

    // MARK: - Edge Snap

    pub fn load_edge_snap_enabled(&self) -> bool {
        self.load_setting("edge_snap_enabled")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    pub fn save_edge_snap_enabled(&self, enabled: bool) {
        self.save_setting("edge_snap_enabled", if enabled { "true" } else { "false" });
    }

    // MARK: - Platform Cookies

    pub fn save_platform_cookies(&self, cookies: &str) -> io::Result<()> {
        let _lock = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
        let data = serde_json::json!({
            "CookieHeader": cookies,
            "SavedAt": chrono::Utc::now().to_rfc3339()
        });
        let path = self.data_dir().join("platform_cookies.json");
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, serde_json::to_string(&data)?)?;
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, "file not created after write"));
        }
        Ok(())
    }

    pub fn load_platform_cookies(&self) -> Option<String> {
        let path = self.data_dir().join("platform_cookies.json");
        if !path.exists() { return None; }
        let json = fs::read_to_string(path).ok()?;
        let doc: serde_json::Value = serde_json::from_str(&json).ok()?;
        let saved_at = doc.get("SavedAt")?.as_str()?;
        if let Ok(saved) = chrono::DateTime::parse_from_rfc3339(saved_at) {
            let age = chrono::Utc::now().signed_duration_since(saved);
            if age.num_days() > 7 { return None; }
        }
        doc.get("CookieHeader")?.as_str().map(String::from)
    }

    pub fn has_saved_platform_cookies(&self) -> bool {
        self.data_dir().join("platform_cookies.json").exists()
    }

    pub fn clear_platform_cookies(&self) {
        let path = self.data_dir().join("platform_cookies.json");
        fs::remove_file(path).ok();
    }

    pub fn clear_all(&self) -> io::Result<()> {
        let dir = self.data_dir();
        if dir.exists() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                if entry.path().extension().map_or(false, |e| e == "json") {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        Ok(())
    }

 pub fn is_first_launch(&self) -> bool {
        !self.load_onboarding_completed()
    }

    pub fn load_onboarding_completed(&self) -> bool {
        let path = self.settings_path();
        let Ok(content) = std::fs::read_to_string(&path) else {
            return false;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else {
            return false;
        };
        value.get("onboarding_completed").and_then(|v| v.as_bool()).unwrap_or(false)
    }

    pub fn save_onboarding_completed(&self) {
        let _lock = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
        let path = self.settings_path();
        let mut value = match std::fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str::<serde_json::Value>(&content).unwrap_or_default(),
            Err(_) => serde_json::Value::Object(serde_json::Map::new()),
        };
        value["onboarding_completed"] = serde_json::Value::Bool(true);
        if let Ok(json) = serde_json::to_string_pretty(&value) {
            let _ = std::fs::write(&path, json);
        }
    }

    // MARK: - Generic Settings

    pub fn load_setting(&self, key: &str) -> Option<String> {
        let path = self.settings_path();
        if !path.exists() { return None; }
        let json = fs::read_to_string(path).ok()?;
        let settings: serde_json::Value = serde_json::from_str(&json).ok()?;
        settings.get(key)?.as_str().map(String::from)
    }

    pub fn save_setting(&self, key: &str, value: &str) {
        let _lock = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
        let path = self.settings_path();
        let mut settings: serde_json::Value = if path.exists() {
            fs::read_to_string(&path).ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(serde_json::json!({}))
        } else {
            serde_json::json!({})
        };
        settings[key] = serde_json::Value::String(value.to_string());
        fs::create_dir_all(path.parent().unwrap()).ok();
        if let Ok(json) = serde_json::to_string(&settings) {
            let _ = fs::write(&path, &json);
        }
    }
}
