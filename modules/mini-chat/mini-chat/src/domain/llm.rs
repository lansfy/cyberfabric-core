//! Domain-level LLM value types.
//!
//! Provider-agnostic types for LLM request construction. These are pure data
//! types with no infrastructure dependencies. Provider adapters in `infra::llm`
//! consume these types and map them to wire formats.

use modkit_macros::domain_model;
use serde::Serialize;

// ════════════════════════════════════════════════════════════════════════════
// Message types
// ════════════════════════════════════════════════════════════════════════════

/// A role in the conversation.
#[domain_model]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    System,
}

/// A content part within a message.
#[domain_model]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { file_id: String },
}

/// A single message in the conversation.
#[domain_model]
#[derive(Debug, Clone)]
pub struct LlmMessage {
    pub role: Role,
    pub content: Vec<ContentPart>,
}

impl LlmMessage {
    /// Create a user message with text content.
    #[must_use]
    pub fn user(text: impl Into<String>) -> Self {
        LlmMessage {
            role: Role::User,
            content: vec![ContentPart::Text { text: text.into() }],
        }
    }

    /// Create an assistant message with text content.
    #[must_use]
    pub fn assistant(text: impl Into<String>) -> Self {
        LlmMessage {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: text.into() }],
        }
    }

    /// Create a user message with text and an image.
    #[must_use]
    pub fn user_with_image(text: impl Into<String>, file_id: impl Into<String>) -> Self {
        LlmMessage {
            role: Role::User,
            content: vec![
                ContentPart::Text { text: text.into() },
                ContentPart::Image {
                    file_id: file_id.into(),
                },
            ],
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Tool types
// ════════════════════════════════════════════════════════════════════════════

/// A provider-agnostic tool descriptor.
///
/// Each adapter maps supported tools to its wire format and silently drops
/// unsupported ones with a `debug!` log.
#[domain_model]
#[derive(Debug, Clone)]
pub enum LlmTool {
    /// Server-side file search (provider manages execution).
    FileSearch { vector_store_ids: Vec<String> },
    /// Server-side web search (provider manages execution).
    WebSearch,
    /// Generic function tool (for providers supporting function calling).
    Function {
        name: String,
        description: String,
        parameters: serde_json::Value,
    },
}

// ════════════════════════════════════════════════════════════════════════════
// Context assembly input types
// ════════════════════════════════════════════════════════════════════════════

/// Minimal message representation for context assembly input.
///
/// Decouples context assembly from ORM entities — only carries the fields
/// needed for LLM prompt construction.
#[domain_model]
#[derive(Debug, Clone)]
pub struct ContextMessage {
    pub role: Role,
    pub content: String,
}
