//! 设备指纹生成
//!
//! 为每个凭证生成独立的 Machine ID，避免多账号共用同一指纹被检测。

use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// 为每个凭证生成独立的 Machine ID
///
/// 策略：
/// 1. 基于 profile_arn 或 client_id 生成唯一标识
/// 2. 添加时间变化因子（每小时变化）避免指纹固化
/// 3. 每个凭证独立 Machine ID，防止多账号共用指纹被检测
pub fn generate_machine_id_from_credentials(
    profile_arn: Option<&str>,
    client_id: Option<&str>,
) -> String {
    // 1. 确定唯一标识
    let unique_key = profile_arn
        .filter(|s| !s.is_empty())
        .or(client_id.filter(|s| !s.is_empty()))
        .unwrap_or("default-kiro-key");

    // 2. 添加时间变化因子（每小时变化）
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let hour_slot = now / 3600;

    // 3. SHA256 哈希
    let mut hasher = Sha256::new();
    hasher.update(unique_key.as_bytes());
    hasher.update(&hour_slot.to_le_bytes());
    let hash = hasher.finalize();

    // 4. 格式化为 64 字符十六进制字符串
    format!("{:x}", hash)
}

/// 获取原始 Machine ID（未哈希）
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

/// 生成设备指纹 (Machine ID 的 SHA256)
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

        // 相同的 profile_arn 应该生成相同的 machine_id（在同一小时内）
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
}
