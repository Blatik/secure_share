use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::File;
use crate::file_transfer::read_file_as_bytes;

const API_BASE: &str = "https://lead-gen-api-fsoe.shuttle.app";

#[derive(Serialize, Deserialize)]
struct UploadResponse {
    id: String,
    expires_at: String,
}

#[derive(Serialize, Deserialize)]
struct DownloadResponse {
    data: String,
    filename: String,
    mime_type: String,
}

pub struct StorageService;

impl StorageService {
    pub async fn upload_data(data: Vec<u8>, filename: String, mime_type: String) -> Result<String, String> {
        // Encode to base64
        let base64_data = base64::encode(&data);

        // Upload
        let response = Request::post(&format!("{}/upload", API_BASE))
            .json(&serde_json::json!({ 
                "data": base64_data,
                "filename": filename,
                "mime_type": mime_type
            }))
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Upload failed: {}", response.status()));
        }

        let resp_json: UploadResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(resp_json.id)
    }

    pub async fn upload_file(file: File) -> Result<String, String> {
        // Read file
        let data = read_file_as_bytes(file.clone()).await?;
        let filename = file.name();
        let mime_type = file.type_();
        
        Self::upload_data(data, filename, mime_type).await
    }

    pub async fn download_file(id: &str) -> Result<(Vec<u8>, String, String), String> {
        let response = Request::get(&format!("{}/download/{}", API_BASE, id))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Download failed: {}", response.status()));
        }

        let resp_json: DownloadResponse = response.json().await.map_err(|e| e.to_string())?;
        
        // Decode base64
        let data = base64::decode(&resp_json.data).map_err(|e| e.to_string())?;
        Ok((data, resp_json.filename, resp_json.mime_type))
    }
}

// Add base64 module (since we don't have the crate in frontend yet, or we can add it)
mod base64 {
    use base64::{engine::general_purpose, Engine as _};

    pub fn encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::STANDARD.decode(data)
    }
}
