use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage};

#[derive(Clone)]
pub(crate) struct ShortMemory {
    system: Option<ChatCompletionRequestSystemMessage>,
    history: Vec<ChatCompletionRequestMessage>,
}

impl ShortMemory {
    pub fn new() -> Self {
        Self {
            system: None,
            history: Vec::new(),
        }
    }

    pub fn append(&mut self, message: ChatCompletionRequestMessage) {
        match message {
            ChatCompletionRequestMessage::System(system) => {
                self.system = Some(system);
            }
            _ => {
                self.history.push(message);
            }
        }
    }

    pub fn messages(&self) -> Vec<ChatCompletionRequestMessage> {
        let mut messages = Vec::new();
        if let Some(system) = &self.system {
            messages.push(ChatCompletionRequestMessage::System(system.clone()));
        }
        messages.extend(self.history.iter().cloned());
        messages
    }
}

#[cfg(test)]
mod tests {
    use async_openai::types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestUserMessage,
    };

    use super::*;

    fn initialize_short_memory() -> ShortMemory {
        let mut short_memory = ShortMemory::new();

        let system_message = ChatCompletionRequestSystemMessage::default();
        let user = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage::default());
        let assistant = ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessage::default(),
        );

        short_memory.append(ChatCompletionRequestMessage::System(system_message.clone()));
        short_memory.append(user.clone());
        short_memory.append(assistant.clone());

        short_memory
    }

    #[test]
    fn test_short_memory_append() {
        let short_memory = initialize_short_memory();

        let system_message = ChatCompletionRequestSystemMessage::default();
        let user = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage::default());
        let assistant = ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessage::default(),
        );

        assert_eq!(short_memory.system, Some(system_message));
        assert_eq!(short_memory.history, vec![user.clone(), assistant.clone()]);
    }

    #[test]
    fn test_short_memory_messages() {
        let short_memory = initialize_short_memory();

        let system =
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage::default());
        let user = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage::default());
        let assistant = ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessage::default(),
        );

        assert_eq!(short_memory.messages(), vec![system, user, assistant]);
    }
}
