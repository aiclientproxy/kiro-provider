//! 设备指纹生成
//!
//! 为每个凭证生成独立的 Machine ID（与 AIClient-2-API 保持一致）。
//! 采用静态 UUID 方案，不随时间变化，这是目前最稳定的实现。

use sha2::{Digest, Sha256};

/// 为每个凭证生成独立的 Machine ID（与 AIClient-2-API 保持一致）
///
/// 策略：
/// 1. 优先级：uuid > profile_arn > client_id > 默认值
/// 2. 生成静态的 SHA256 哈希，不包含时间因子
/// 3. 每个凭证独立 Machine ID，防止多账号共用指纹被检测
pub fn generate_machine_id_from_credentials(
    profile_arn: Option<&str>,
    client_id: Option<&str>,
) -> String {
    generate_machine_id_from_credentials_with_uuid(None, profile_arn, client_id)
}

/// 带 UUID 参数的 Machine ID 生成函数（与 AIClient-2-API 完全一致）
///
/// 优先级：uuid > profileArn > clientId > 默认值
/// 生成静态的 SHA256 哈希，不包含时间因子
pub fn generate_machine_id_from_credentials_with_uuid(
    uuid: Option<&str>,
    profile_arn: Option<&str>,
    client_id: Option<&str>,
) -> String {
    // 优先级：uuid > profileArn > clientId > 默认值（与 AIClient-2-API 一致）
    let unique_key = uuid
        .filter(|s| !s.is_empty())
        .or(profile_arn.filter(|s| !s.is_empty()))
        .or(client_id.filter(|s| !s.is_empty()))
        .unwrap_or("KIRO_DEFAULT_MACHINE");

    // 静态哈希，不添加时间因子（与 AIClient-2-API 保持一致）
    let mut hasher = Sha256::new();
    hasher.update(unique_key.as_bytes());
    let hash = hasher.finalize();

    // 格式化为 64 字符十六进制字符串
    format!("{:x}", hash)
}

/// 获取原始 Machine ID（未哈希）- 保留用于兼容
pub fn get_raw_machine_id() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| {
                s.lines()
                    .find(|l| l.contains("IOPlatformUUID"))
                    .and_then(|l| l.split('=').nth(1))
                    .map(|s| s.trim().trim_matches('"').to_lowercase())
            })
    }

    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/etc/machine-id")
            .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
            .ok()
            .map(|s| s.trim().to_lowercase())
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("wmic")
            .args(["csproduct", "get", "UUID"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| {
                s.lines()
                    .skip(1)
                    .find(|l| !l.trim().is_empty())
                    .map(|s| s.trim().to_lowercase())
            })
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// 生成设备指纹 (Machine ID 的 SHA256) - 保留用于兼容
pub fn get_device_fingerprint() -> String {
    let raw_id = get_raw_machine_id()
        .unwrap_or_else(|| "00000000-0000-0000-0000-000000000000".to_string());

    let mut hasher = Sha256::new();
    hasher.update(raw_id.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_machine_id() {
        let id1 = generate_machine_id_from_credentials(Some("arn:aws:iam::123"), None);
        let id2 = generate_machine_id_from_credentials(Some("arn:aws:iam::456"), None);

        // 不同的 profile_arn 应该生成不同的 machine_id
        assert_ne!(id1, id2);

        // 相同的 profile_arn 应该生成相同的 machine_id（静态方案，不随时间变化）
        let id3 = generate_machine_id_from_credentials(Some("arn:aws:iam::123"), None);
        assert_eq!(id1, id3);
    }

    #[test]
    fn test_machine_id_format() {
        let id = generate_machine_id_from_credentials(Some("test"), None);
        // SHA256 输出 64 字符十六进制
        assert_eq!(id.len(), 64);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_uuid_priority() {
        // UUID 优先级最高
        let id_with_uuid = generate_machine_id_from_credentials_with_uuid(
            Some("test-uuid"),
            Some("arn:aws:iam::123"),
            Some("client-123"),
        );
        let id_uuid_only = generate_machine_id_from_credentials_with_uuid(
            Some("test-uuid"),
            None,
            None,
        );
        assert_eq!(id_with_uuid, id_uuid_only);

        // 没有 UUID 时使用 profile_arn
        let id_no_uuid = generate_machine_id_from_credentials_with_uuid(
            None,
            Some("arn:aws:iam::123"),
            Some("client-123"),
        );
        let id_arn_only = generate_machine_id_from_credentials(
            Some("arn:aws:iam::123"),
            None,
        );
        assert_eq!(id_no_uuid, id_arn_only);
    }
}
