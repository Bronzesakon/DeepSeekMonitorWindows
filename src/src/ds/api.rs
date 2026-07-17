use crate::ds::models::*;
use reqwest::Client;
use std::sync::{Mutex, atomic::AtomicBool};

#[derive(Debug, Clone)]
pub enum ApiError {
    Unauthorized,
    RateLimited,
    ServerError(u16),
    HttpError(u16),
    NetworkError(String),
    DecodingError(String),
    InvalidResponse,
    PlatformError(u16, String),
    PlatformUnauthorized,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "认证无效或已过期，请重新登录"),
            ApiError::RateLimited => write!(f, "请求过于频繁，请稍后重试"),
            ApiError::ServerError(c) => write!(f, "服务器错误 ({})", c),
            ApiError::HttpError(c) => write!(f, "HTTP 错误 ({})", c),
            ApiError::NetworkError(_) => write!(f, "网络连接失败，请检查网络设置"),
            ApiError::DecodingError(m) => write!(f, "数据解析错误: {}", m),
            ApiError::InvalidResponse => write!(f, "服务器返回无效响应"),
            ApiError::PlatformError(c, m) => write!(f, "平台API HTTP {}: {}", c, m),
            ApiError::PlatformUnauthorized => write!(f, "登录无效或已过期，请重新登录"),
        }
    }
}

pub struct ApiState {
    pub platform_token: Mutex<Option<String>>,
    pub platform_cookies: Mutex<Option<String>>,
    pub is_refreshing: AtomicBool,
    pub last_data: Mutex<Option<DashboardData>>,
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            platform_token: Mutex::new(None),
            platform_cookies: Mutex::new(None),
            is_refreshing: AtomicBool::new(false),
            last_data: Mutex::new(None),
        }
    }

    pub fn has_platform_session(&self) -> bool {
        self.platform_token.lock().unwrap_or_else(|e| e.into_inner()).is_some()
    }

    pub fn cached_data(&self) -> Option<DashboardData> {
        self.last_data.lock().ok().and_then(|data| data.clone())
    }

    pub fn cache_data(&self, data: &DashboardData) {
        if let Ok(mut cached) = self.last_data.lock() {
            *cached = Some(data.clone());
        }
    }
}

impl Default for ApiState {
    fn default() -> Self {
        Self::new()
    }
}

const PLATFORM_BASE_URL: &str = "https://platform.deepseek.com";

pub async fn fetch_platform_cost(
    client: &Client,
    token: &str,
    cookies: Option<&str>,
    year: i32,
    month: u32,
) -> Result<PlatformCostResponse, ApiError> {
    let url = format!(
        "{}/api/v0/usage/cost?month={}&year={}",
        PLATFORM_BASE_URL, month, year
    );
    let mut req = client
        .get(&url)
        .bearer_auth(token)
        .header("Accept", "application/json")
        .header("Origin", PLATFORM_BASE_URL)
        .header("Referer", format!("{}/usage", PLATFORM_BASE_URL));

    if let Some(c) = cookies {
        req = req.header("Cookie", c);
    }

    let resp = req.send().await.map_err(|e| {
        crate::debug_log!("[api] fetch_platform_cost 网络错误: {} - {}", url, e);
        ApiError::NetworkError(e.to_string())
    })?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| {
        crate::debug_log!("[api] fetch_platform_cost 读取响应失败: {}", e);
        ApiError::NetworkError(e.to_string())
    })?;

    if !status.is_success() {
        let short = if body.len() > 500 { &body[..500] } else { &body };
        crate::debug_log!("[api] fetch_platform_cost HTTP {}: {}", status.as_u16(), short);
        if status.as_u16() == 401 || status.as_u16() == 403 {
            return Err(ApiError::PlatformUnauthorized);
        }
        return Err(ApiError::PlatformError(status.as_u16(), short.to_string()));
    }

    let resp: PlatformCostResponse = serde_json::from_str(&body)
        .map_err(|e| {
            crate::debug_log!("[api] fetch_platform_cost 解析失败: {} body: {}", e, &body[..body.len().min(500)]);
            ApiError::DecodingError(format!("platform_cost parse: {}", e))
        })?;
    if resp.code != 0 {
        crate::debug_log!("[api] fetch_platform_cost code != 0: {}, body: {}", resp.code, &body[..body.len().min(500)]);
        return Err(ApiError::PlatformUnauthorized);
    }
    Ok(resp)
}

pub async fn fetch_platform_amount(
    client: &Client,
    token: &str,
    cookies: Option<&str>,
    year: i32,
    month: u32,
) -> Result<PlatformAmountResponse, ApiError> {
    let url = format!(
        "{}/api/v0/usage/amount?month={}&year={}",
        PLATFORM_BASE_URL, month, year
    );
    let mut req = client
        .get(&url)
        .bearer_auth(token)
        .header("Accept", "application/json")
        .header("Origin", PLATFORM_BASE_URL)
        .header("Referer", format!("{}/usage", PLATFORM_BASE_URL));

    if let Some(c) = cookies {
        req = req.header("Cookie", c);
    }

    let resp = req.send().await.map_err(|e| {
        crate::debug_log!("[api] fetch_platform_amount 网络错误: {} - {}", url, e);
        ApiError::NetworkError(e.to_string())
    })?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| {
        crate::debug_log!("[api] fetch_platform_amount 读取响应失败: {}", e);
        ApiError::NetworkError(e.to_string())
    })?;

    if !status.is_success() {
        let short = if body.len() > 500 { &body[..500] } else { &body };
        crate::debug_log!("[api] fetch_platform_amount HTTP {}: {}", status.as_u16(), short);
        if status.as_u16() == 401 || status.as_u16() == 403 {
            return Err(ApiError::PlatformUnauthorized);
        }
        return Err(ApiError::PlatformError(status.as_u16(), short.to_string()));
    }

    let resp: PlatformAmountResponse = serde_json::from_str(&body)
        .map_err(|e| {
            crate::debug_log!("[api] fetch_platform_amount 解析失败: {} body: {}", e, &body[..body.len().min(500)]);
            ApiError::DecodingError(format!("platform_amount parse: {}", e))
        })?;
    if resp.code != 0 {
        crate::debug_log!("[api] fetch_platform_amount code != 0: {}, body: {}", resp.code, &body[..body.len().min(500)]);
        return Err(ApiError::PlatformUnauthorized);
    }
    Ok(resp)
}

pub async fn fetch_user_summary(
    client: &Client,
    token: &str,
    cookies: Option<&str>,
) -> Result<UserSummaryResponse, ApiError> {
    let url = format!("{}/api/v0/users/get_user_summary", PLATFORM_BASE_URL);
    let mut req = client
        .get(&url)
        .bearer_auth(token)
        .header("Accept", "application/json")
        .header("Origin", PLATFORM_BASE_URL)
        .header("Referer", format!("{}/usage", PLATFORM_BASE_URL));

    if let Some(c) = cookies {
        req = req.header("Cookie", c);
    }

    let resp = req.send().await.map_err(|e| {
        crate::debug_log!("[api] fetch_user_summary 网络错误: {} - {}", url, e);
        ApiError::NetworkError(e.to_string())
    })?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| {
        crate::debug_log!("[api] fetch_user_summary 读取响应失败: {}", e);
        ApiError::NetworkError(e.to_string())
    })?;

    if !status.is_success() {
        let short = if body.len() > 500 { &body[..500] } else { &body };
        crate::debug_log!("[api] fetch_user_summary HTTP {}: {}", status.as_u16(), short);
        if status.as_u16() == 401 || status.as_u16() == 403 {
            return Err(ApiError::PlatformUnauthorized);
        }
        return Err(ApiError::PlatformError(status.as_u16(), short.to_string()));
    }

    let resp: UserSummaryResponse = serde_json::from_str(&body)
        .map_err(|e| {
            crate::debug_log!("[api] fetch_user_summary 解析失败: {} body: {}", e, &body[..body.len().min(500)]);
            ApiError::DecodingError(format!("user_summary parse: {}", e))
        })?;
    if resp.code != 0 {
        crate::debug_log!("[api] fetch_user_summary code != 0: {}", resp.code);
        return Err(ApiError::PlatformUnauthorized);
    }
    Ok(resp)
}

// MARK: - Model name matching (same logic as C# IsProModel / IsFlashModel)

pub(crate) fn is_pro_model(model: &str) -> bool {
    let lower = model.to_lowercase();
    lower.contains("pro") || lower.contains("reasoner")
}

pub(crate) fn is_flash_model(model: &str) -> bool {
    let lower = model.to_lowercase();
    lower.contains("flash") || lower.contains("chat")
}

/// Find first flash-compatible model (matches "flash" or "chat" in name)
pub(crate) fn find_flash_model<'a>(models: &'a [crate::ds::models::PlatformModelData]) -> Option<&'a crate::ds::models::PlatformModelData> {
    models.iter().find(|m| is_flash_model(&m.model))
}

// MARK: - Data Processing (ported from DashboardViewModel)

pub(crate) fn format_number(n: i32) -> String {
    let is_negative = n < 0;
    let abs_n = n.unsigned_abs();
    let mut s = String::new();
    let n_str = abs_n.to_string();
    let len = n_str.len();
    for (i, c) in n_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            s.push(',');
        }
        s.push(c);
    }
    if is_negative {
        format!("-{}", s)
    } else {
        s
    }
}

pub(crate) fn format_date_short(date_str: &str) -> String {
    if date_str.len() >= 10 {
        date_str[5..10].to_string()
    } else {
        date_str.to_string()
    }
}

/// Build daily usage from platform API amount response
pub(crate) fn build_daily_from_platform(
    days: &[crate::ds::models::PlatformDayData],
    is_pro: bool,
) -> Vec<crate::ds::models::ModelDailyUsagePoint> {
    use crate::ds::models::ModelDailyUsagePoint;

    let result: Vec<_> = days.iter()
        .filter_map(|day| {
            // For flash models, prefer "flash" over "chat"
            let model_data = if is_pro {
                day.data.iter().find(|m| is_pro_model(&m.model))
            } else {
                find_flash_model(&day.data)
            }?;
            let mut total_tokens: i32 = 0;
            let mut cache_hit: i32 = 0;
            let mut cache_miss: i32 = 0;
            let mut output: i32 = 0;
            let mut request_count: i32 = 0;

            for entry in &model_data.usage {
                let val = entry.amount.parse::<i32>().unwrap_or(0);
                match entry.usage_type.as_str() {
                    "PROMPT_TOKEN" => total_tokens = val,
                    "PROMPT_CACHE_HIT_TOKEN" => cache_hit = val,
                    "PROMPT_CACHE_MISS_TOKEN" => cache_miss = val,
                    "RESPONSE_TOKEN" => output = val,
                    "REQUEST" => request_count = val,
                    _ => {}
                }
            }

            if total_tokens == 0 {
                total_tokens = cache_hit + cache_miss + output;
            }

            if total_tokens == 0 && request_count == 0 {
                return None;
            }

            Some(ModelDailyUsagePoint {
                date: day.date.clone(),
                label: format_date_short(&day.date),
                total_tokens,
                input_cache_hit_tokens: cache_hit,
                input_cache_miss_tokens: cache_miss,
                output_tokens: output,
                request_count,
                cost_in_cents: 0,
            })
        })
        .collect();

    result
}
