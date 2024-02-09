use crate::{
    chat_message::ChatMessage,
    state::{ChatState, LocalChatState},
};
use askama::Template;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::{str::FromStr, sync::Arc};
use tracing::warn;

pub async fn upgrade_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<LocalChatState>>,
) -> Response {
    ws.on_upgrade(move |socket| async {
        handle_socket(socket, app_state).await;
    })
}

async fn handle_socket(socket: WebSocket, app_state: Arc<LocalChatState>) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = app_state.tx.subscribe();
    let broadcast_tx = app_state.tx.clone();

    // receive broadcast messages and send them to the current client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender
                .send(Message::Text(msg.render().unwrap()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(_) => return, // client disconnected
            };

            if let Message::Text(msg) = msg {
                let chat_message = ChatMessage::from_str(&msg);

                match chat_message {
                    Ok(chat_message) => {
                        // add to history
                        app_state.add_message(chat_message.clone());
                        // send ChatMessage to broadcast channel
                        let _ = broadcast_tx.send(chat_message);
                    }
                    Err(_) => warn!("invalid message: {:?}", chat_message),
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
