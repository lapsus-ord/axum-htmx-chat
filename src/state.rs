use crate::chat_message::ChatMessage;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct LocalChatState {
    messages: Arc<Mutex<Vec<ChatMessage>>>,
}

pub trait ChatState {
    fn add_message(&self, message: ChatMessage);
    fn get_history(&self, n: Option<usize>) -> Vec<ChatMessage>;
}

impl ChatState for LocalChatState {
    fn add_message(&self, message: ChatMessage) {
        self.messages.lock().expect("mutex poisoned").push(message);
    }

    fn get_history(&self, n: Option<usize>) -> Vec<ChatMessage> {
        let last_n_messages = n.unwrap_or(50);

        self.messages
            .lock()
            .expect("mutex poisoned")
            .iter()
            .take(last_n_messages)
            .cloned()
            .collect()
    }
}
