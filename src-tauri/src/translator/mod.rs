//! 协议转换模块
//!
//! 实现 Anthropic/OpenAI → CodeWhisperer 和 CodeWhisperer → Anthropic SSE 的转换。

pub mod anthropic_to_cw;
pub mod cw_to_anthropic;
pub mod openai_to_cw;

pub use anthropic_to_cw::convert_anthropic_to_codewhisperer;
pub use cw_to_anthropic::CwToAnthropicTranslator;
pub use openai_to_cw::convert_openai_to_codewhisperer;

/// 模型名称映射
pub fn map_model_name(model: &str) -> String {
    let mappings = [
        ("claude-opus-4-5", "claude-opus-4.5"),
        ("claude-opus-4-5-20251101", "claude-opus-4.5"),
        ("claude-haiku-4-5", "claude-haiku-4.5"),
        ("claude-haiku-4-5-20251001", "claude-haiku-4.5"),
        ("claude-sonnet-4-5", "CLAUDE_SONNET_4_5_20250929_V1_0"),
        ("claude-sonnet-4-5-20250929", "CLAUDE_SONNET_4_5_20250929_V1_0"),
        ("claude-sonnet-4-5-20250514", "CLAUDE_SONNET_4_5_20250514_V1_0"),
        ("claude-sonnet-4-20250514", "CLAUDE_SONNET_4_20250514_V1_0"),
        ("claude-3-7-sonnet-20250219", "CLAUDE_3_7_SONNET_20250219_V1_0"),
        ("claude-3-5-sonnet-20241022", "CLAUDE_3_7_SONNET_20250219_V1_0"),
    ];

    for (from, to) in mappings {
        if model.contains(from) {
            return to.to_string();
        }
    }

    model.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_mapping() {
        assert_eq!(map_model_name("claude-opus-4-5"), "claude-opus-4.5");
        assert_eq!(
            map_model_name("claude-sonnet-4-5-20250929"),
            "CLAUDE_SONNET_4_5_20250929_V1_0"
        );
        assert_eq!(map_model_name("unknown-model"), "unknown-model");
    }
}
