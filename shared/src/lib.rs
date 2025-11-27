use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalMessage {
    /// Request to join a session or create one if it doesn't exist (implicitly, or we can separate create/join)
    /// For simplicity: JoinSession(session_id)
    JoinSession(String),
    
    /// SDP Offer from initiator
    Offer {
        session_id: String,
        sdp: String,
    },
    
    /// SDP Answer from peer
    Answer {
        session_id: String,
        sdp: String,
    },
    
    /// ICE Candidate
    IceCandidate {
        session_id: String,
        candidate: String,
    },
    
    /// Public Key for E2EE (X25519)
    PublicKey {
        session_id: String,
        pubkey: String, // Base64 encoded
    },
    
    /// Error message
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionId(pub String);
