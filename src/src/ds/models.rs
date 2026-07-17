use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub currency: String,
    pub total_balance: String,
    pub granted_balance: String,
    pub topped_up_balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeepSeekModel {
    Flash,
    Pro,
}

impl DeepSeekModel {
    pub fn api_model_name(&self) -> &str {
        match self {
            DeepSeekModel::Flash => "deepseek-chat",
            DeepSeekModel::Pro => "deepseek-reasoner",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            DeepSeekModel::Flash => "V4 Flash",
            DeepSeekModel::Pro => "V4 Pro",
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            DeepSeekModel::Flash => "Flash",
            DeepSeekModel::Pro => "Pro",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsageSummary {
    pub model: DeepSeekModel,
    pub total_tokens: i32,
    pub cost_in_cents: i32,
    pub total_tokens_formatted: String,
    pub cost_formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDailyUsagePoint {
    pub date: String,
    pub label: String,
    pub total_tokens: i32,
    pub input_cache_hit_tokens: i32,
    pub input_cache_miss_tokens: i32,
    pub output_tokens: i32,
    pub request_count: i32,
    pub cost_in_cents: i32,
}

// Platform API models

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformUsageEntry {
    #[serde(rename = "type")]
    pub usage_type: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformModelData {
    pub model: String,
    pub usage: Vec<PlatformUsageEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDayData {
    pub date: String,
    pub data: Vec<PlatformModelData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformBizDataContent {
    pub total: Vec<PlatformModelData>,
    pub days: Vec<PlatformDayData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCostResponse {
    pub code: i32,
    pub data: Option<PlatformCostInnerData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformAmountResponse {
    pub code: i32,
    pub data: Option<PlatformAmountInnerData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCostInnerData {
    pub biz_code: i32,
    pub biz_data: Option<Vec<PlatformBizDataContent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformAmountInnerData {
    pub biz_code: i32,
    pub biz_data: Option<PlatformBizDataContent>,
}

// User Summary API models (platform.deepseek.com/api/v0/users/get_user_summary)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryResponse {
    pub code: i32,
    pub data: Option<UserSummaryInnerData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryInnerData {
    pub biz_code: i32,
    pub biz_data: Option<UserSummaryBizData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryBizData {
    pub normal_wallets: Vec<UserSummaryWallet>,
    pub bonus_wallets: Vec<UserSummaryWallet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryWallet {
    pub currency: String,
    pub balance: String,
    pub token_estimation: String,
}

// Dashboard aggregate data sent to frontend

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    // Balance
    pub is_account_available: bool,
    pub total_balance: f64,
    pub granted_balance: f64,
    pub topped_up_balance: f64,
    pub balance_info: Option<BalanceInfo>,

    // Usage
    pub flash_usage: Option<ModelUsageSummary>,
    pub pro_usage: Option<ModelUsageSummary>,
    pub flash_daily_usage: Vec<ModelDailyUsagePoint>,
    pub pro_daily_usage: Vec<ModelDailyUsagePoint>,

    // Platform data
    pub current_day_cost: f64,
    pub current_month_cost: f64,
    pub current_day_requests: i32,
    pub current_day_flash_tokens: i64,
    pub current_day_pro_tokens: i64,

    // State
    pub has_platform_session: bool,
    pub is_first_launch: bool,
    pub last_updated: String,
    pub error_message: Option<String>,
    pub warning_message: Option<String>,
}