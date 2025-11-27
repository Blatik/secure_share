# Secure Share - Storage Backend

Rust backend для зберігання зашифрованих файлів з автоматичним видаленням через 24 години.

## Технології

- **Shuttle** - Deployment platform
- **Axum** - Web framework
- **DashMap** - Concurrent in-memory storage
- **Base64** - Encoding/decoding

## API Endpoints

### 1. Upload File
```http
POST /upload
Content-Type: application/json

{
  "data": "base64_encoded_encrypted_data"
}
```

**Response:**
```json
{
  "id": "uuid-v4",
  "expires_at": "2024-11-25T01:28:10Z"
}
```

### 2. Download File
```http
GET /download/:id
```

**Response:**
```json
{
  "data": "base64_encoded_encrypted_data"
}
```

**Status Codes:**
- `200 OK` - File found
- `404 NOT FOUND` - File doesn't exist
- `410 GONE` - File expired

### 3. Delete File
```http
POST /delete/:id
```

**Response:**
- `204 NO CONTENT` - File deleted
- `404 NOT FOUND` - File doesn't exist

### 4. Health Check
```http
GET /health
```

**Response:**
```
OK
```

## Features

- ✅ In-memory storage (швидко, але не персистентно)
- ✅ Автоматичне видалення через 24 години
- ✅ CORS enabled для frontend
- ✅ Base64 encoding/decoding
- ✅ UUID для унікальних ID
- ✅ Cleanup task кожну годину

## Local Development

```bash
# Run locally
cargo shuttle run

# Deploy to Shuttle
cargo shuttle deploy
```

## Environment

Server runs on port assigned by Shuttle (usually 8000 locally).

## Usage Example

```javascript
// Upload
const response = await fetch('http://localhost:8000/upload', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    data: btoa(encryptedData) // Base64 encode
  })
});
const { id, expires_at } = await response.json();

// Download
const downloadResp = await fetch(`http://localhost:8000/download/${id}`);
const { data } = await downloadResp.json();
const decryptedData = atob(data); // Base64 decode

// Delete (optional)
await fetch(`http://localhost:8000/delete/${id}`, { method: 'POST' });
```

## Security Notes

- Файли зберігаються в пам'яті (не на диску)
- Автоматичне видалення через 24 години
- Підтримує тільки зашифровані дані (шифрування на клієнті)
- CORS дозволяє доступ з будь-якого origin (для MVP)

## Future Improvements

- [ ] Persistent storage (Redis/PostgreSQL)
- [ ] Rate limiting
- [ ] File size limits
- [ ] Authentication
- [ ] Metrics/monitoring
- [ ] Compression
