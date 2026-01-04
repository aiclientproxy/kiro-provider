//! OpenAI → CodeWhisperer 转换

use super::anthropic_to_cw::{
    AssistantResponseConfig, CodeWhispererRequest, ConversationState, CurrentMessage,
    ResponseStyle, UserInputMessage,
};
use super::map_model_name;
use serde_json::Value;

/// 将 OpenAI 请求转换为 CodeWhisperer 格式
pub fn convert_openai_to_codewhisperer(
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

    // 提取系统提示（OpenAI 格式中系统消息是 messages 数组的一部分）
    let system_prompt = if let Some(msgs) = messages {
        msgs.iter()
            .find(|m| m["role"].as_str() == Some("system"))
            .and_then(|m| m["content"].as_str())
            .map(String::from)
    } else {
        None
    };

    // 提取用户消息内容
    let content = if let Some(msgs) = messages {
        msgs.iter()
            .filter(|m| m["role"].as_str() == Some("user"))
            .last()
            .and_then(|m| {
                if let Some(s) = m["content"].as_str() {
                    Some(s.to_string())
                } else if let Some(arr) = m["content"].as_array() {
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

    // 提取参数
    let max_tokens = request["max_tokens"]
        .as_u64()
        .or_else(|| request["max_completion_tokens"].as_u64())
        .map(|v| v as u32);
    let temperature = request["temperature"].as_f64().map(|v| v as f32);

    // 构建历史消息（排除系统消息和最后一条用户消息）
    let history = if let Some(msgs) = messages {
        let non_system: Vec<_> = msgs
            .iter()
            .filter(|m| m["role"].as_str() != Some("system"))
            .cloned()
            .collect();

        let len = non_system.len();
        if len > 1 {
            Some(non_system.into_iter().take(len - 1).collect())
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
    fn test_convert_openai_request() {
        let request = serde_json::json!({
            "model": "gpt-4",
            "max_tokens": 1024,
            "messages": [
                {"role": "system", "content": "You are helpful."},
                {"role": "user", "content": "Hello!"}
            ]
        });

        let result = convert_openai_to_codewhisperer(&request, None);

        assert_eq!(
            result.conversation_state.current_message.user_input_message.content,
            "Hello!"
        );
        assert!(result.assistant_response_config.response_style.is_some());
    }
}
