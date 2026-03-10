//! Pure context assembly for LLM requests.
//!
//! Assembles system instructions, conversation messages, and tool definitions
//! from domain inputs. No I/O, no async — all data is gathered beforehand.

use modkit_macros::domain_model;

use crate::domain::llm::{ContextMessage, LlmMessage, LlmTool, Role};

/// All inputs needed to assemble the LLM request context.
#[domain_model]
pub struct ContextInput<'a> {
    /// System prompt from the model catalog (via preflight).
    pub system_prompt: &'a str,
    /// Guard instruction appended when `web_search` is enabled.
    pub web_search_guard: &'a str,
    /// Guard instruction appended when `file_search` is enabled.
    pub file_search_guard: &'a str,
    /// Thread summary content (if exists).
    pub thread_summary: Option<&'a str>,
    /// Recent messages from DB, already in chronological order.
    pub recent_messages: &'a [ContextMessage],
    /// Current user message text.
    pub user_message: &'a str,
    /// Whether `web_search` tool is enabled for this request.
    pub web_search_enabled: bool,
    /// Whether `file_search` tool is enabled for this request.
    pub file_search_enabled: bool,
    /// Vector store IDs for `file_search` (empty = no `file_search` tool).
    pub vector_store_ids: &'a [String],
}

/// Output of context assembly — ready to feed into `LlmRequestBuilder`.
#[domain_model]
pub struct AssembledContext {
    /// System instructions (None if empty).
    pub system_instructions: Option<String>,
    /// Conversation messages in normative order.
    pub messages: Vec<LlmMessage>,
    /// Tool definitions to include in the request.
    pub tools: Vec<LlmTool>,
}

/// Assemble the LLM request context from gathered domain inputs.
///
/// Normative order:
/// 1. System prompt + tool guards → `system_instructions`
/// 2. Thread summary (as user message with prefix) → first message
/// 3. Recent messages (role-mapped) → middle messages
/// 4. Current user message → last message
#[must_use]
pub fn assemble_context(input: &ContextInput<'_>) -> AssembledContext {
    // ── System instructions ──
    let system_instructions = build_system_instructions(
        input.system_prompt,
        input.web_search_enabled,
        input.web_search_guard,
        input.file_search_enabled,
        input.file_search_guard,
    );

    // ── Messages ──
    let mut messages = Vec::new();

    // Thread summary as first message (if present)
    if let Some(summary) = input.thread_summary {
        messages.push(LlmMessage::user(format!("[Thread Summary]\n{summary}")));
    }

    // Recent messages (role-mapped)
    for msg in input.recent_messages {
        match msg.role {
            Role::User => messages.push(LlmMessage::user(&msg.content)),
            Role::Assistant => messages.push(LlmMessage::assistant(&msg.content)),
            Role::System => {
                // System messages are not included in context assembly.
            }
        }
    }

    // Current user message (always last)
    messages.push(LlmMessage::user(input.user_message));

    // ── Tools ──
    let mut tools = Vec::new();
    if input.file_search_enabled && !input.vector_store_ids.is_empty() {
        tools.push(LlmTool::FileSearch {
            vector_store_ids: input.vector_store_ids.to_vec(),
        });
    }
    if input.web_search_enabled {
        tools.push(LlmTool::WebSearch);
    }

    AssembledContext {
        system_instructions,
        messages,
        tools,
    }
}

/// Build system instructions from base prompt + conditional guard strings.
/// Returns `None` if the result would be empty.
fn build_system_instructions(
    system_prompt: &str,
    web_search_enabled: bool,
    web_search_guard: &str,
    file_search_enabled: bool,
    file_search_guard: &str,
) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();

    if !system_prompt.is_empty() {
        parts.push(system_prompt);
    }
    if web_search_enabled && !web_search_guard.is_empty() {
        parts.push(web_search_guard);
    }
    if file_search_enabled && !file_search_guard.is_empty() {
        parts.push(file_search_guard);
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_message(role: Role, content: &str) -> ContextMessage {
        ContextMessage {
            role,
            content: content.to_owned(),
        }
    }

    // 5.6: empty system prompt + no tools → system_instructions: None, tools: []
    #[test]
    fn empty_system_prompt_no_tools() {
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: false,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        assert!(result.system_instructions.is_none());
        assert!(result.tools.is_empty());
        assert_eq!(result.messages.len(), 1);
    }

    // 5.7: system prompt + web_search enabled → guard appended
    #[test]
    fn system_prompt_with_web_search_guard() {
        let result = assemble_context(&ContextInput {
            system_prompt: "You are helpful.",
            web_search_guard: "Use web_search only if needed.",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: true,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        let instructions = result.system_instructions.unwrap();
        assert!(instructions.contains("You are helpful."));
        assert!(instructions.contains("Use web_search only if needed."));
    }

    // 5.8: system prompt + file_search enabled → guard appended
    #[test]
    fn system_prompt_with_file_search_guard() {
        let result = assemble_context(&ContextInput {
            system_prompt: "You are helpful.",
            web_search_guard: "",
            file_search_guard: "Use file_search for documents.",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: false,
            file_search_enabled: true,
            vector_store_ids: &["vs-1".to_owned()],
        });
        let instructions = result.system_instructions.unwrap();
        assert!(instructions.contains("You are helpful."));
        assert!(instructions.contains("Use file_search for documents."));
    }

    // 5.9: both guards appended when both tools enabled
    #[test]
    fn both_guards_appended() {
        let result = assemble_context(&ContextInput {
            system_prompt: "Base prompt.",
            web_search_guard: "web guard",
            file_search_guard: "file guard",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: true,
            file_search_enabled: true,
            vector_store_ids: &["vs-1".to_owned()],
        });
        let instructions = result.system_instructions.unwrap();
        assert!(instructions.contains("Base prompt."));
        assert!(instructions.contains("web guard"));
        assert!(instructions.contains("file guard"));
    }

    // 5.10: thread summary present → included as first message with prefix
    #[test]
    fn thread_summary_included_as_first_message() {
        let recent = vec![make_message(Role::User, "prior question")];
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: Some("Summary of prior conversation."),
            recent_messages: &recent,
            user_message: "new question",
            web_search_enabled: false,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        // First message should be the thread summary
        assert_eq!(result.messages.len(), 3); // summary + recent + current
        let first_content = &result.messages[0].content;
        match &first_content[0] {
            crate::domain::llm::ContentPart::Text { text } => {
                assert!(text.contains("[Thread Summary]"));
                assert!(text.contains("Summary of prior conversation."));
            }
            crate::domain::llm::ContentPart::Image { .. } => {
                panic!("Expected text content")
            }
        }
    }

    // 5.11: no thread summary → messages start with recent history
    #[test]
    fn no_thread_summary_starts_with_recent() {
        let recent = vec![
            make_message(Role::User, "first"),
            make_message(Role::Assistant, "response"),
        ];
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &recent,
            user_message: "second",
            web_search_enabled: false,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        assert_eq!(result.messages.len(), 3); // 2 recent + current
    }

    // 5.12: recent messages mapped by role (user/assistant), system role skipped
    #[test]
    fn system_role_skipped() {
        let recent = vec![
            make_message(Role::User, "hello"),
            make_message(Role::System, "system msg"),
            make_message(Role::Assistant, "hi"),
        ];
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &recent,
            user_message: "bye",
            web_search_enabled: false,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        // system message skipped: 2 recent (user+assistant) + 1 current = 3
        assert_eq!(result.messages.len(), 3);
    }

    // 5.13: current user message always last
    #[test]
    fn current_user_message_is_last() {
        let recent = vec![make_message(Role::Assistant, "prior")];
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &recent,
            user_message: "current input",
            web_search_enabled: false,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        let last = result.messages.last().unwrap();
        match &last.content[0] {
            crate::domain::llm::ContentPart::Text { text } => {
                assert_eq!(text, "current input");
            }
            crate::domain::llm::ContentPart::Image { .. } => {
                panic!("Expected text content")
            }
        }
    }

    // 5.14: tools vec populated correctly for file_search + web_search combinations
    #[test]
    fn tools_populated_correctly() {
        // Both enabled with vector store
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: true,
            file_search_enabled: true,
            vector_store_ids: &["vs-123".to_owned()],
        });
        assert_eq!(result.tools.len(), 2);
        assert!(matches!(&result.tools[0], LlmTool::FileSearch { .. }));
        assert!(matches!(&result.tools[1], LlmTool::WebSearch));

        // file_search enabled but no vector store IDs → no file_search tool
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: false,
            file_search_enabled: true,
            vector_store_ids: &[],
        });
        assert!(result.tools.is_empty());

        // Only web_search
        let result = assemble_context(&ContextInput {
            system_prompt: "",
            web_search_guard: "",
            file_search_guard: "",
            thread_summary: None,
            recent_messages: &[],
            user_message: "hello",
            web_search_enabled: true,
            file_search_enabled: false,
            vector_store_ids: &[],
        });
        assert_eq!(result.tools.len(), 1);
        assert!(matches!(&result.tools[0], LlmTool::WebSearch));
    }
}
