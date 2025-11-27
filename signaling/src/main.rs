use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use dashmap::DashMap;
use futures::{sink::SinkExt, stream::StreamExt};
use shared::SignalMessage;
use std::{sync::Arc, net::SocketAddr};
use tokio::sync::broadcast;
use tracing::{info, warn};

// State to hold active sessions.
// Key: SessionId
// Value: Broadcast channel for that session to relay messages
struct AppState {
    sessions: DashMap<String, broadcast::Sender<SignalMessage>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        sessions: DashMap::new(),
    });

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    
    // 1. Wait for JoinSession
    let mut session_id = String::new();
    let mut my_id = uuid::Uuid::new_v4().to_string();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Ok(signal_msg) = serde_json::from_str::<SignalMessage>(&text) {
                if let SignalMessage::JoinSession(sid) = signal_msg {
                    session_id = sid;
                    info!("Client {} joined session {}", my_id, session_id);
                    break;
                }
            }
        }
    }

    if session_id.is_empty() {
        return; // Client disconnected or didn't join
    }

    // 2. Subscribe to session broadcast
    let tx = state.sessions.entry(session_id.clone())
        .or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(100);
            tx
        })
        .value()
        .clone();
    
    let mut rx = tx.subscribe();

    // 3. Main loop
    loop {
        tokio::select! {
            // Receive from WebSocket and broadcast
            result = receiver.next() => {
                match result {
                    Some(Ok(msg)) => {
                        if let Message::Text(text) = msg {
                            // We assume the client sends valid SignalMessages
                            // In a real app, we might validate or inject the sender_id here
                            // For now, just relay raw text if it parses
                            if let Ok(parsed_msg) = serde_json::from_str::<SignalMessage>(&text) {
                                info!("Relaying message in session {}: {:?}", session_id, parsed_msg);
                                // Avoid echoing back to self?
                                // The broadcast channel will send to everyone including us.
                                // We can't easily filter on the receiver side of the channel without wrapping the message.
                                // Let's just send it. Client will filter.
                                let _ = tx.send(parsed_msg);
                            }
                        }
                    }
                    Some(Err(e)) => {
                        warn!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        break; // Disconnected
                    }
                }
            }

            // Receive from Broadcast and send to WebSocket
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        // Serialize and send
                        if let Ok(text) = serde_json::to_string(&msg) {
                            if sender.send(Message::Text(text)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Broadcast error: {}", e);
                        break;
                    }
                }
            }
        }
    }
    
    // Cleanup if needed (DashMap handles removing empty entries? No, we might want to cleanup empty sessions)
    // For MVP, we leave them.
    info!("Client {} disconnected from session {}", my_id, session_id);
}
