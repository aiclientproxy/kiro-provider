//! 风控逻辑
//!
//! 实现 Kiro 特有的风控适配，包括版本伪装、User-Agent 构造等。

use std::process::Command;

/// 运行时信息
pub struct RuntimeInfo {
    pub os_name: String,
    pub os_version: String,
    pub node_version: String,
}

/// 获取系统运行时信息
pub fn get_system_runtime_info() -> RuntimeInfo {
    RuntimeInfo {
        os_name: get_os_name(),
        os_version: get_os_version(),
        node_version: "20.18.0".to_string(), // 模拟 Node.js 版本
    }
}

/// 获取操作系统名称
fn get_os_name() -> String {
    if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else if cfg!(target_os = "windows") {
        "windows".to_string()
    } else {
        "other".to_string()
    }
}

/// 获取操作系统版本
fn get_os_version() -> String {
    #[cfg(target_os = "macos")]
    {
        Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "14.0".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("uname")
            .arg("-r")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "5.15.0".to_string())
    }

    #[cfg(target_os = "windows")]
    {
        "10.0".to_string()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        "1.0".to_string()
    }
}

/// 获取 Kiro IDE 版本号
///
/// 尝试从 Kiro.app 的 Info.plist 读取实际版本，失败时使用默认值
pub fn get_kiro_version() -> String {
    #[cfg(target_os = "macos")]
    {
        let kiro_paths = [
            "/Applications/Kiro.app/Contents/Info.plist",
            &format!(
                "{}/Applications/Kiro.app/Contents/Info.plist",
                dirs::home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            ),
        ];

        for plist_path in &kiro_paths {
            if let Ok(output) = Command::new("defaults")
                .args(["read", plist_path, "CFBundleShortVersionString"])
                .output()
            {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    let version = version.trim();
                    if !version.is_empty() {
                        return version.to_string();
                    }
                }
            }
        }
    }

    // 默认版本号
    "0.1.25".to_string()
}

/// 构建 Social Auth Token 刷新 User-Agent
pub fn build_social_auth_user_agent(kiro_version: &str, machine_id: &str) -> String {
    format!("KiroIDE-{}-{}", kiro_version, machine_id)
}

/// 构建 IdC Auth Token 刷新 User-Agent
pub fn build_idc_auth_user_agent(kiro_version: &str, machine_id: &str) -> String {
    format!(
        "aws-sdk-js/3.738.0 ua/2.1 os/other lang/js api/sso-oidc#3.738.0 m/E KiroIDE-{}-{}",
        kiro_version, machine_id
    )
}

/// 构建 API 调用 User-Agent
pub fn build_api_user_agent(
    os_name: &str,
    node_version: &str,
    kiro_version: &str,
    machine_id: &str,
) -> String {
    format!(
        "aws-sdk-js/1.0.0 ua/2.1 os/{} lang/js md/nodejs#{} api/codewhispererruntime#1.0.0 m/E KiroIDE-{}-{}",
        os_name, node_version, kiro_version, machine_id
    )
}

/// 构建 x-amz-user-agent 头
pub fn build_x_amz_user_agent(kiro_version: &str, machine_id: &str) -> String {
    format!("aws-sdk-js/1.0.0 KiroIDE-{}-{}", kiro_version, machine_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_kiro_version() {
        let version = get_kiro_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_build_user_agents() {
        let kiro_version = "0.1.25";
        let machine_id = "abc123";

        let social_ua = build_social_auth_user_agent(kiro_version, machine_id);
        assert!(social_ua.contains("KiroIDE"));
        assert!(social_ua.contains(kiro_version));

        let idc_ua = build_idc_auth_user_agent(kiro_version, machine_id);
        assert!(idc_ua.contains("aws-sdk-js"));
        assert!(idc_ua.contains("sso-oidc"));

        let api_ua = build_api_user_agent("macos", "20.18.0", kiro_version, machine_id);
        assert!(api_ua.contains("codewhispererruntime"));
    }
}
