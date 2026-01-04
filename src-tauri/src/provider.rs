//! Kiro Provider 核心实现
//!
//! 实现凭证管理、模型支持检查等核心功能。

use crate::credentials::{AcquiredCredential, KiroCredentials, ValidationResult};
use crate::fingerprint::generate_machine_id_from_credentials;
use crate::risk_control::get_kiro_version;
use crate::token_refresh::TokenRefreshResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// 模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
    pub family: Option<String>,
    pub context_length: Option<u32>,
    pub supports_vision: bool,
    pub supports_tools: bool,
}

/// Provider 错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderError {
    pub error_type: String,
    pub message: String,
    pub status_code: Option<u16>,
    pub retryable: bool,
    pub cooldown_seconds: Option<u64>,
}

/// 凭证存储
lazy_static::lazy_static! {
    static ref CREDENTIALS: Arc<RwLock<HashMap<String, KiroCredentials>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

/// 列出支持的模型
pub fn list_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "claude-sonnet-4-5-20250514".to_string(),
            display_name: "Claude Sonnet 4.5".to_string(),
            family: Some("sonnet".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-opus-4-5-20251101".to_string(),
            display_name: "Claude Opus 4.5".to_string(),
            family: Some("opus".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-3-5-sonnet-20241022".to_string(),
            display_name: "Claude 3.5 Sonnet".to_string(),
            family: Some("sonnet".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-3-5-haiku-20241022".to_string(),
            display_name: "Claude 3.5 Haiku".to_string(),
            family: Some("haiku".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
        ModelInfo {
            id: "claude-3-7-sonnet-20250219".to_string(),
            display_name: "Claude 3.7 Sonnet".to_string(),
            family: Some("sonnet".to_string()),
            context_length: Some(200000),
            supports_vision: true,
            supports_tools: true,
        },
    ]
}

/// 检查是否支持某个模型
pub fn supports_model(model: &str) -> bool {
    model.starts_with("claude-")
}

/// 获取凭证
pub async fn acquire_credential(model: &str) -> Result<AcquiredCredential> {
    if !supports_model(model) {
        anyhow::bail!("不支持的模型: {}", model);
    }

    let creds = CREDENTIALS.read().await;

    // 查找健康的凭证
    let healthy_creds: Vec<_> = creds
        .iter()
        .filter(|(_, c)| c.is_healthy)
        .collect();

    if healthy_creds.is_empty() {
        anyhow::bail!("没有可用的健康凭证");
    }

    // 选择第一个健康凭证
    let (id, credential) = healthy_creds.first().unwrap();

    let token = credential
        .access_token
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("凭证没有有效的 access_token"))?;

    // 构建请求头
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", token));
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    // 添加 Kiro 特有的头部
    let machine_id = generate_machine_id_from_credentials(
        credential.profile_arn.as_deref(),
        credential.client_id.as_deref(),
    );
    let kiro_version = get_kiro_version();
    headers.insert(
        "x-amz-user-agent".to_string(),
        format!("aws-sdk-js/1.0.0 KiroIDE-{}-{}", kiro_version, machine_id),
    );

    // 构建 base URL
    let region = credential.region.as_deref().unwrap_or("us-east-1");
    let base_url = format!("https://codewhisperer.{}.amazonaws.com", region);

    Ok(AcquiredCredential {
        id: (*id).clone(),
        name: credential.name.clone(),
        auth_type: "oauth".to_string(),
        base_url: Some(base_url),
        headers,
        metadata: HashMap::new(),
    })
}

/// 释放凭证
pub async fn release_credential(credential_id: &str, result: serde_json::Value) -> Result<()> {
    let mut creds = CREDENTIALS.write().await;

    if let Some(credential) = creds.get_mut(credential_id) {
        credential.usage_count += 1;

        if let Some(error) = result.get("error") {
            credential.error_count += 1;
            credential.last_error = error.get("message").and_then(|m| m.as_str()).map(String::from);

            if error.get("mark_unhealthy").and_then(|v| v.as_bool()).unwrap_or(false) {
                credential.is_healthy = false;
                warn!("凭证标记为不健康: {}", credential_id);
            }
        } else {
            credential.is_healthy = true;
            credential.last_error = None;
            debug!("凭证使用成功: {}", credential_id);
        }
    }

    Ok(())
}

/// 验证凭证
pub async fn validate_credential(credential_id: &str) -> Result<ValidationResult> {
    let creds = CREDENTIALS.read().await;

    if let Some(credential) = creds.get(credential_id) {
        let has_token = credential.access_token.is_some() || credential.refresh_token.is_some();

        Ok(ValidationResult {
            valid: has_token && credential.is_healthy,
            message: if has_token {
                Some("凭证有效".to_string())
            } else {
                Some("缺少有效的 token".to_string())
            },
            details: HashMap::new(),
        })
    } else {
        Ok(ValidationResult {
            valid: false,
            message: Some("凭证不存在".to_string()),
            details: HashMap::new(),
        })
    }
}

/// 刷新 Token
pub async fn refresh_token(credential_id: &str) -> Result<TokenRefreshResult> {
    let mut creds = CREDENTIALS.write().await;

    if let Some(credential) = creds.get_mut(credential_id) {
        // 调用 token_refresh 模块
        let result = crate::token_refresh::refresh_token(credential).await?;

        // 更新凭证
        credential.access_token = Some(result.access_token.clone());
        if let Some(ref rt) = result.refresh_token {
            credential.refresh_token = Some(rt.clone());
        }
        credential.expire = result.expires_at.map(|dt| dt.to_rfc3339());
        credential.is_healthy = true;
        credential.last_error = None;

        info!("Token 刷新成功: {}", credential_id);
        Ok(result)
    } else {
        anyhow::bail!("凭证不存在: {}", credential_id)
    }
}

/// 创建凭证
pub async fn create_credential(auth_type: &str, config: serde_json::Value) -> Result<String> {
    if auth_type != "oauth" {
        anyhow::bail!("不支持的认证类型: {}", auth_type);
    }

    let kiro_config: KiroCredentials = serde_json::from_value(config)?;

    // 验证必要字段
    if kiro_config.refresh_token.is_none() {
        anyhow::bail!("缺少必要的 refresh_token");
    }

    // 生成凭证 ID
    let credential_id = uuid::Uuid::new_v4().to_string();

    // 存储凭证
    let mut creds = CREDENTIALS.write().await;
    creds.insert(credential_id.clone(), kiro_config);

    info!("创建凭证成功: {}", credential_id);
    Ok(credential_id)
}

/// 转换请求
pub async fn transform_request(request: serde_json::Value) -> Result<serde_json::Value> {
    // Kiro 使用特殊的 CodeWhisperer 格式
    // 这里可以调用 translator 模块进行转换
    Ok(request)
}

/// 转换响应
pub async fn transform_response(response: serde_json::Value) -> Result<serde_json::Value> {
    // 响应转换
    Ok(response)
}

/// 应用风控
pub async fn apply_risk_control(
    request: &mut serde_json::Value,
    _credential_id: &str,
) -> Result<()> {
    // Kiro 的风控主要通过 machine_id 实现，已在 acquire_credential 中处理
    Ok(())
}

/// 解析错误
pub fn parse_error(status: u16, body: &str) -> Option<ProviderError> {
    match status {
        401 => Some(ProviderError {
            error_type: "authentication".to_string(),
            message: "Token 已过期或无效".to_string(),
            status_code: Some(status),
            retryable: true,
            cooldown_seconds: Some(0),
        }),
        403 => Some(ProviderError {
            error_type: "authorization".to_string(),
            message: "权限不足".to_string(),
            status_code: Some(status),
            retryable: false,
            cooldown_seconds: None,
        }),
        429 => Some(ProviderError {
            error_type: "rate_limit".to_string(),
            message: "请求过于频繁".to_string(),
            status_code: Some(status),
            retryable: true,
            cooldown_seconds: Some(60),
        }),
        500..=599 => Some(ProviderError {
            error_type: "server_error".to_string(),
            message: format!("服务器错误: {}", body),
            status_code: Some(status),
            retryable: true,
            cooldown_seconds: Some(10),
        }),
        _ => None,
    }
}
