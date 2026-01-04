//! 凭证数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kiro OAuth 凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KiroCredentials {
    /// 凭证名称
    #[serde(default)]
    pub name: Option<String>,
    /// Access Token
    pub access_token: Option<String>,
    /// Refresh Token
    pub refresh_token: Option<String>,
    /// Client ID (IdC 认证需要)
    pub client_id: Option<String>,
    /// Client Secret (IdC 认证需要)
    pub client_secret: Option<String>,
    /// Profile ARN (Social 认证需要)
    pub profile_arn: Option<String>,
    /// Region
    #[serde(default = "default_region")]
    pub region: Option<String>,
    /// 认证方式 (social 或 idc)
    #[serde(default = "default_auth_method")]
    pub auth_method: Option<String>,
    /// Client ID Hash
    pub client_id_hash: Option<String>,
    /// 过期时间 (RFC3339 格式)
    pub expire: Option<String>,
    /// 最后刷新时间
    pub last_refresh: Option<String>,
    /// 是否健康
    #[serde(default = "default_true")]
    pub is_healthy: bool,
    /// 使用次数
    #[serde(default)]
    pub usage_count: u64,
    /// 错误次数
    #[serde(default)]
    pub error_count: u64,
    /// 最后错误信息
    #[serde(default)]
    pub last_error: Option<String>,
}

fn default_region() -> Option<String> {
    Some("us-east-1".to_string())
}

fn default_auth_method() -> Option<String> {
    Some("social".to_string())
}

fn default_true() -> bool {
    true
}

impl Default for KiroCredentials {
    fn default() -> Self {
        Self {
            name: None,
            access_token: None,
            refresh_token: None,
            client_id: None,
            client_secret: None,
            profile_arn: None,
            region: default_region(),
            auth_method: default_auth_method(),
            client_id_hash: None,
            expire: None,
            last_refresh: None,
            is_healthy: true,
            usage_count: 0,
            error_count: 0,
            last_error: None,
        }
    }
}

/// 获取的凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquiredCredential {
    /// 凭证 ID
    pub id: String,
    /// 凭证名称
    #[serde(default)]
    pub name: Option<String>,
    /// 认证方式
    pub auth_type: String,
    /// Base URL（如果有）
    #[serde(default)]
    pub base_url: Option<String>,
    /// 请求头（Key-Value 对）
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// 额外元数据
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 凭证验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否有效
    pub valid: bool,
    /// 消息
    #[serde(default)]
    pub message: Option<String>,
    /// 额外信息
    #[serde(default)]
    pub details: HashMap<String, serde_json::Value>,
}
