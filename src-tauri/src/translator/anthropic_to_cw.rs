//! Anthropic → CodeWhisperer 转换

use super::map_model_name;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CodeWhisperer 请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeWhispererRequest {
    pub conversation_state: ConversationState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    pub source: String,
    pub assistant_response_config: AssistantResponseConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationState {
    pub current_message: CurrentMessage,
    pub chat_trigger_type: String,
    pub user_intent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentMessage {
    pub user_input_message: UserInputMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInputMessage {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_input_message_context: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantResponseConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_style: Option<ResponseStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt_user_customization: Option<String>,
}

/// 将 Anthropic 请求转换为 CodeWhisperer 格式
pub fn convert_anthropic_to_codewhisperer(
    request: &Value,
    profile_arn: Option<String>,
) -> CodeWhispererRequest {
    // 提取模型并映射
    let _model = request["model"]
        .as_str()
        .map(map_model_name)
        .unwrap_or_else(|| "claude-sonnet-4.5".to_string());

    // 提取消息内容
    let messages = request["messages"].as_array();
    let content = if let Some(msgs) = messages {
        // 获取最后一条用户消息
        msgs.iter()
            .filter(|m| m["role"].as_str() == Some("user"))
            .last()
            .and_then(|m| {
                // 处理 content 可能是字符串或数组的情况
                if let Some(s) = m["content"].as_str() {
                    Some(s.to_string())
                } else if let Some(arr) = m["content"].as_array() {
                    // 提取文本内容
                    Some(
                        arr.iter()
                            .filter_map(|c| {
                                if c["type"].as_str() == Some("text") {
                                    c["text"].as_str().map(String::from)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        String::new()
    };

    // 提取系统提示
    let system_prompt = request["system"].as_str().map(String::from);

    // 提取参数
    let max_tokens = request["max_tokens"].as_u64().map(|v| v as u32);
    let temperature = request["temperature"].as_f64().map(|v| v as f32);

    // 构建历史消息
    let history = if let Some(msgs) = messages {
        if msgs.len() > 1 {
            Some(
                msgs.iter()
                    .take(msgs.len() - 1)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    } else {
        None
    };

    CodeWhispererRequest {
        conversation_state: ConversationState {
            current_message: CurrentMessage {
                user_input_message: UserInputMessage {
                    content,
                    user_input_message_context: None,
                },
            },
            chat_trigger_type: "MANUAL".to_string(),
            user_intent: "CHAT".to_string(),
            customization_arn: None,
            history,
        },
        profile_arn,
        source: "CHAT".to_string(),
        assistant_response_config: AssistantResponseConfig {
            max_output_tokens: max_tokens,
            temperature,
            response_style: system_prompt.map(|s| ResponseStyle {
                system_prompt_user_customization: Some(s),
            }),
        },
        tools: request["tools"].as_array().cloned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_request() {
        let request = serde_json::json!({
            "model": "claude-sonnet-4-5-20250514",
            "max_tokens": 1024,
            "messages": [
                {"role": "user", "content": "Hello, world!"}
            ]
        });

        let result = convert_anthropic_to_codewhisperer(&request, None);

        assert_eq!(
            result.conversation_state.current_message.user_input_message.content,
            "Hello, world!"
        );
        assert_eq!(result.assistant_response_config.max_output_tokens, Some(1024));
    }

    #[test]
    fn test_convert_with_system_prompt() {
        let request = serde_json::json!({
            "model": "claude-sonnet-4-5-20250514",
            "system": "You are a helpful assistant.",
            "messages": [
                {"role": "user", "content": "Hi"}
            ]
        });

        let result = convert_anthropic_to_codewhisperer(&request, Some("arn:aws:iam::123".to_string()));

        assert!(result.profile_arn.is_some());
        assert!(result.assistant_response_config.response_style.is_some());
    }
}
