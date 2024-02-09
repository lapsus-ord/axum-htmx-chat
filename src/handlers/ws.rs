use crate::{
    chat_message::ChatMessage,
    state::{ChatState, LocalChatState},
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use std::str::FromStr;
use tracing::warn;

pub async fn upgrade_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<LocalChatState>,
) -> Response {
    ws.on_upgrade(|socket| async {
        handle_socket(socket, app_state).await;
    })
}

async fn handle_socket(mut socket: WebSocket, app_state: LocalChatState) {
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => return, // client disconnected
        };

        if let Message::Text(msg) = msg {
            let chat_message = ChatMessage::from_str(&msg);

            if let Ok(chat_message) = chat_message {
                app_state.add_message(chat_message.clone());

                let response = Message::Text(chat_message.to_string());

                if socket.send(response).await.is_err() {
                    // client disconnected
                    return;
                }
            } else {
                warn!("invalid message: {:?}", chat_message);
            }
        }
    }
}
