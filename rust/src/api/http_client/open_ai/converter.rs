pub use crate::api::http_client::open_ai::errors::{ChatErrors, ChatResults};
pub use async_openai_wasm::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
};
pub use serde::{Deserialize, Serialize};
pub use serde_json;

/// Conversations History
#[derive(Serialize, Deserialize)]
pub struct ChatLog {
    pub contents: Vec<ChatCompletionRequestMessage>,
}
impl ChatLog {
    /// Converting the Chat Histories from fromtend into a list of tuplets wuth role definition and text.    
    /// The result can be used directly in the Chat-service.
    pub fn msg_convertion(chat_log: Vec<(String, String)>) -> Self {
        let messages: Vec<ChatCompletionRequestMessage> = chat_log
            .into_iter()
            .map(|(role, content)| match role.trim() {
                "system" => ChatCompletionRequestSystemMessageArgs::default()
                    .content(content)
                    .build()
                    .unwrap()
                    .into(),
                "assistant" => ChatCompletionRequestAssistantMessageArgs::default()
                    .content(content)
                    .build()
                    .unwrap()
                    .into(),
                _ => ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()
                    .unwrap()
                    .into(),
            })
            .collect();
        ChatLog { contents: messages }
    }

    /// Serialize Data into JSON-format
    pub fn serialize(&self) -> ChatResults<String> {
        let parsed = serde_json::to_string(&self)?;
        Ok(parsed)
    }
}