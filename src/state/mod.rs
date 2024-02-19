use crate::chat_message::ChatMessage;

pub mod local;

pub trait ChatState {
    fn add_message(&self, message: ChatMessage);
    fn get_history(&self, n: Option<usize>) -> Vec<ChatMessage>;
}
