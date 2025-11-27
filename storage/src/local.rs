use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;

// Stored file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredFile {
    id: String,
    data: Vec<u8>,
    filename: String,
    mime_type: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Serialize, Deserialize)]
struct UploadRequest {
    data: String, // Base64 encoded encrypted data
    filename: String,
    mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadResponse {
    id: String,
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DownloadResponse {
    data: String, // Base64 encoded encrypted data
    filename: String,
    mime_type: String,
}

// Application state
struct AppState {
    files: DashMap<String, StoredFile>,
}

impl AppState {
    fn new() -> Self {
        Self {
            files: DashMap::new(),
        }
    }

    // Cleanup expired files
    fn cleanup_expired(&self) {
        let now = Utc::now();
        self.files.retain(|_, file| file.expires_at > now);
    }
}

// Upload endpoint
async fn upload_file(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UploadRequest>,
) -> Result<Json<UploadResponse>, StatusCode> {
    // Decode base64
    let data = base64::decode(&payload.data)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Generate unique ID
    let id = Uuid::new_v4().to_string();
    
    // Set expiration (24 hours from now)
    let created_at = Utc::now();
    let expires_at = created_at + chrono::Duration::hours(24);

    // Store file
    let stored_file = StoredFile {
        id: id.clone(),
        data,
        filename: payload.filename,
        mime_type: payload.mime_type,
        created_at,
        expires_at,
    };

    state.files.insert(id.clone(), stored_file);
    
    // Cleanup old files
    state.cleanup_expired();

    info!("File uploaded: {} (expires: {})", id, expires_at);

    Ok(Json(UploadResponse {
        id,
        expires_at,
    }))
}

// Download endpoint
async fn download_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DownloadResponse>, StatusCode> {
    // Get file
    let file = state.files.get(&id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if expired
    if file.expires_at < Utc::now() {
        drop(file);
        state.files.remove(&id);
        return Err(StatusCode::GONE);
    }

    // Encode to base64
    let data = base64::encode(&file.data);

    info!("File downloaded: {}", id);

    Ok(Json(DownloadResponse { 
        data,
        filename: file.filename.clone(),
        mime_type: file.mime_type.clone(),
    }))
}

// Delete endpoint (optional, for cleanup after download)
async fn delete_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> StatusCode {
    if state.files.remove(&id).is_some() {
        info!("File deleted: {}", id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Health check
async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState::new());

    // Spawn cleanup task
    let cleanup_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600)); // Every hour
        loop {
            interval.tick().await;
            cleanup_state.cleanup_expired();
            info!("Cleanup task executed");
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/upload", post(upload_file))
        .route("/download/:id", get(download_file))
        .route("/delete/:id", post(delete_file))
        .with_state(state)
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    info!("🚀 Storage server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Add base64 module
mod base64 {
    use base64::{engine::general_purpose, Engine as _};

    pub fn encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::STANDARD.decode(data)
    }
}
