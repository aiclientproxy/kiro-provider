//! CodeWhisperer → Anthropic SSE 转换

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// AWS Event Stream 事件类型
#[derive(Debug, Clone)]
pub enum AwsEventType {
    MessageStart,
    ContentBlockStart { content_type: String },
    ContentBlockDelta { delta: String },
    ContentBlockStop,
    MessageDelta { stop_reason: Option<String>, usage: Usage },
    MessageStop,
}

/// 使用量统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Anthropic SSE 事件
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnthropicSseEvent {
    MessageStart {
        message: MessageStartData,
    },
    ContentBlockStart {
        index: u32,
        content_block: ContentBlock,
    },
    ContentBlockDelta {
        index: u32,
        delta: Delta,
    },
    ContentBlockStop {
        index: u32,
    },
    MessageDelta {
        delta: MessageDelta,
        usage: Usage,
    },
    MessageStop,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageStartData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub role: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Delta {
    TextDelta { text: String },
    ToolUse { id: String, name: String, input: Value },
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
}

/// CodeWhisperer → Anthropic SSE 转换器
pub struct CwToAnthropicTranslator {
    message_id: String,
    model: String,
    current_index: u32,
    input_tokens: u32,
    output_tokens: u32,
}

impl CwToAnthropicTranslator {
    /// 创建新的转换器
    pub fn new(model: &str) -> Self {
        Self {
            message_id: format!("msg_{}", uuid::Uuid::new_v4()),
            model: model.to_string(),
            current_index: 0,
            input_tokens: 0,
            output_tokens: 0,
        }
    }

    /// 转换 AWS Event Stream 事件为 Anthropic SSE 事件
    pub fn translate_event(&mut self, event_type: AwsEventType) -> Vec<AnthropicSseEvent> {
        let mut events = Vec::new();

        match event_type {
            AwsEventType::MessageStart => {
                events.push(AnthropicSseEvent::MessageStart {
                    message: MessageStartData {
                        id: self.message_id.clone(),
                        type_: "message".to_string(),
                        role: "assistant".to_string(),
                        model: self.model.clone(),
                    },
                });
            }
            AwsEventType::ContentBlockStart { content_type } => {
                events.push(AnthropicSseEvent::ContentBlockStart {
                    index: self.current_index,
                    content_block: ContentBlock { type_: content_type },
                });
            }
            AwsEventType::ContentBlockDelta { delta } => {
                events.push(AnthropicSseEvent::ContentBlockDelta {
                    index: self.current_index,
                    delta: Delta::TextDelta { text: delta },
                });
            }
            AwsEventType::ContentBlockStop => {
                events.push(AnthropicSseEvent::ContentBlockStop {
                    index: self.current_index,
                });
                self.current_index += 1;
            }
            AwsEventType::MessageDelta { stop_reason, usage } => {
                self.input_tokens = usage.input_tokens;
                self.output_tokens = usage.output_tokens;
                events.push(AnthropicSseEvent::MessageDelta {
                    delta: MessageDelta { stop_reason },
                    usage: Usage {
                        input_tokens: self.input_tokens,
                        output_tokens: self.output_tokens,
                    },
                });
            }
            AwsEventType::MessageStop => {
                events.push(AnthropicSseEvent::MessageStop);
            }
        }

        events
    }

    /// 格式化为 SSE 字符串
    pub fn format_sse(event: &AnthropicSseEvent) -> String {
        let event_type = match event {
            AnthropicSseEvent::MessageStart { .. } => "message_start",
            AnthropicSseEvent::ContentBlockStart { .. } => "content_block_start",
            AnthropicSseEvent::ContentBlockDelta { .. } => "content_block_delta",
            AnthropicSseEvent::ContentBlockStop { .. } => "content_block_stop",
            AnthropicSseEvent::MessageDelta { .. } => "message_delta",
            AnthropicSseEvent::MessageStop => "message_stop",
        };

        let data = serde_json::to_string(event).unwrap_or_default();
        format!("event: {}\ndata: {}\n\n", event_type, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translator() {
        let mut translator = CwToAnthropicTranslator::new("claude-sonnet-4.5");

        let events = translator.translate_event(AwsEventType::MessageStart);
        assert_eq!(events.len(), 1);

        let events = translator.translate_event(AwsEventType::ContentBlockStart {
            content_type: "text".to_string(),
        });
        assert_eq!(events.len(), 1);

        let events = translator.translate_event(AwsEventType::ContentBlockDelta {
            delta: "Hello".to_string(),
        });
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_format_sse() {
        let event = AnthropicSseEvent::ContentBlockDelta {
            index: 0,
            delta: Delta::TextDelta {
                text: "Hello".to_string(),
            },
        };

        let sse = CwToAnthropicTranslator::format_sse(&event);
        assert!(sse.starts_with("event: content_block_delta"));
        assert!(sse.contains("Hello"));
    }
}
