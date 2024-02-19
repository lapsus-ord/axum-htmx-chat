use super::ChatState;
use crate::chat_message::ChatMessage;
use std::sync::Mutex;
use tokio::sync::broadcast;

pub struct LocalChatState {
    // history of chat messages
    messages: Mutex<Vec<ChatMessage>>,
    // Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<ChatMessage>,
}

impl LocalChatState {
    pub fn new(messages: Vec<ChatMessage>, tx: broadcast::Sender<ChatMessage>) -> Self {
        Self {
            messages: Mutex::new(messages),
            tx,
        }
    }
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
