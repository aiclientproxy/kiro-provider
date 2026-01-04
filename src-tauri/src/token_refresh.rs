//! Token 刷新逻辑
//!
//! 支持 Social Auth 和 IdC Auth 两种认证方式的 Token 刷新。

use crate::credentials::KiroCredentials;
use crate::fingerprint::generate_machine_id_from_credentials;
use crate::risk_control::{
    build_idc_auth_user_agent, build_social_auth_user_agent, get_kiro_version,
};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Token 刷新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshResult {
    /// 新的 access_token
    pub access_token: String,
    /// 新的 refresh_token（如果更新了）
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// 过期时间
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
}

/// Token 刷新响应
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenResponse {
    access_token: Option<String>,
    #[serde(alias = "accessToken")]
    access_token_camel: Option<String>,
    refresh_token: Option<String>,
    #[serde(alias = "refreshToken")]
    refresh_token_camel: Option<String>,
    expires_in: Option<i64>,
    #[serde(alias = "expiresIn")]
    expires_in_camel: Option<i64>,
    profile_arn: Option<String>,
    #[serde(alias = "profileArn")]
    profile_arn_camel: Option<String>,
}

impl TokenResponse {
    fn get_access_token(&self) -> Option<&str> {
        self.access_token
            .as_deref()
            .or(self.access_token_camel.as_deref())
    }

    fn get_refresh_token(&self) -> Option<&str> {
        self.refresh_token
            .as_deref()
            .or(self.refresh_token_camel.as_deref())
    }

    fn get_expires_in(&self) -> Option<i64> {
        self.expires_in.or(self.expires_in_camel)
    }
}

/// 刷新 Token
pub async fn refresh_token(credential: &mut KiroCredentials) -> Result<TokenRefreshResult> {
    // 验证 refresh_token 完整性
    let refresh_token = credential
        .refresh_token
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("缺少 refresh_token"))?;

    if refresh_token.len() < 100 {
        anyhow::bail!(
            "refreshToken 已被截断（长度: {} 字符）。正常的 refreshToken 长度应该在 500+ 字符",
            refresh_token.len()
        );
    }

    let machine_id = generate_machine_id_from_credentials(
        credential.profile_arn.as_deref(),
        credential.client_id.as_deref(),
    );
    let kiro_version = get_kiro_version();
    let auth_method = credential.auth_method.as_deref().unwrap_or("social");

    info!(
        "开始 Token 刷新: auth_method={}, has_client_id={}, has_client_secret={}",
        auth_method,
        credential.client_id.is_some(),
        credential.client_secret.is_some()
    );

    let client = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let result = if auth_method == "idc" {
        refresh_idc_token(&client, credential, &machine_id, &kiro_version).await?
    } else {
        refresh_social_token(&client, credential, &machine_id, &kiro_version).await?
    };

    info!("Token 刷新成功");
    Ok(result)
}

/// Social Auth Token 刷新
async fn refresh_social_token(
    client: &Client,
    credential: &KiroCredentials,
    machine_id: &str,
    kiro_version: &str,
) -> Result<TokenRefreshResult> {
    let region = credential.region.as_deref().unwrap_or("us-east-1");
    let url = format!("https://prod.{}.auth.desktop.kiro.dev/refreshToken", region);

    let refresh_token = credential.refresh_token.as_ref().unwrap();

    debug!("Social Token 刷新: url={}", url);

    let response = client
        .post(&url)
        .header(
            "User-Agent",
            build_social_auth_user_agent(kiro_version, machine_id),
        )
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Encoding", "br, gzip, deflate")
        .header("Content-Type", "application/json")
        .header("Accept-Language", "*")
        .header("Sec-Fetch-Mode", "cors")
        .header("Connection", "close")
        .json(&serde_json::json!({
            "refreshToken": refresh_token
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Social Token 刷新失败: {} - {}", status, body);
    }

    let data: TokenResponse = response.json().await?;

    let access_token = data
        .get_access_token()
        .ok_or_else(|| anyhow::anyhow!("响应中没有 access_token"))?
        .to_string();

    let expires_at = data
        .get_expires_in()
        .map(|secs| Utc::now() + Duration::seconds(secs));

    Ok(TokenRefreshResult {
        access_token,
        refresh_token: data.get_refresh_token().map(String::from),
        expires_at,
    })
}

/// IdC Auth Token 刷新
async fn refresh_idc_token(
    client: &Client,
    credential: &KiroCredentials,
    machine_id: &str,
    kiro_version: &str,
) -> Result<TokenRefreshResult> {
    let region = credential.region.as_deref().unwrap_or("us-east-1");
    let url = format!("https://oidc.{}.amazonaws.com/token", region);

    let client_id = credential
        .client_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("IdC 认证需要 client_id"))?;

    let client_secret = credential
        .client_secret
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("IdC 认证需要 client_secret"))?;

    let refresh_token = credential.refresh_token.as_ref().unwrap();

    debug!("IdC Token 刷新: url={}", url);

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Host", format!("oidc.{}.amazonaws.com", region))
        .header(
            "x-amz-user-agent",
            build_idc_auth_user_agent(kiro_version, machine_id),
        )
        .header("User-Agent", "node")
        .header("Accept", "*/*")
        .header("Connection", "close")
        .json(&serde_json::json!({
            "refreshToken": refresh_token,
            "clientId": client_id,
            "clientSecret": client_secret,
            "grantType": "refresh_token"
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("IdC Token 刷新失败: {} - {}", status, body);
    }

    let data: TokenResponse = response.json().await?;

    let access_token = data
        .get_access_token()
        .ok_or_else(|| anyhow::anyhow!("响应中没有 access_token"))?
        .to_string();

    let expires_at = data
        .get_expires_in()
        .map(|secs| Utc::now() + Duration::seconds(secs));

    Ok(TokenRefreshResult {
        access_token,
        refresh_token: data.get_refresh_token().map(String::from),
        expires_at,
    })
}

/// 检查 Token 是否已过期
pub fn is_token_expired(expire: Option<&str>) -> bool {
    if let Some(expire_str) = expire {
        if let Ok(expires) = DateTime::parse_from_rfc3339(expire_str) {
            let now = Utc::now();
            // 提前5分钟判断为过期
            return expires <= now + Duration::minutes(5);
        }
    }
    // 如果没有过期时间信息，保守地认为可能需要刷新
    true
}

/// 检查 Token 是否即将过期（10 分钟内）
pub fn is_token_expiring_soon(expire: Option<&str>) -> bool {
    if let Some(expire_str) = expire {
        if let Ok(expiry) = DateTime::parse_from_rfc3339(expire_str) {
            let now = Utc::now();
            let threshold = now + Duration::minutes(10);
            return expiry < threshold;
        }
    }
    false
}
