use crate::state::{ChatState, LocalChatState};
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

pub async fn chat_history(State(app_state): State<Arc<LocalChatState>>) -> impl IntoResponse {
    let message_history = app_state
        .get_history(None)
        .iter()
        .map(|m| m.render())
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .join("");

    Html::from(message_history)
}
