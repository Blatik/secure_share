// WebRTC Helper for WASM
// This helper stores DataChannel in a global variable for Rust to access

window._webrtc_datachannel = null;

window.setupWebRTCHelper = function (pc) {
    console.log("Setting up WebRTC helper...");

    // Set up ondatachannel with proper event handling
    pc.ondatachannel = function (event) {
        console.log("🔥 JavaScript: ondatachannel event fired!");
        const channel = event.channel;

        // Store in global variable
        window._webrtc_datachannel = channel;
        console.log("✅ DataChannel stored globally!");

        // Set up event handlers
        channel.onopen = function () {
            console.log("✅ DataChannel opened!");
        };

        channel.onerror = function (error) {
            console.error("❌ DataChannel error:", error);
        };

        channel.onclose = function () {
            console.log("🔒 DataChannel closed");
            window._webrtc_datachannel = null;
        };

        channel.onmessage = function (event) {
            console.log("📨 DataChannel message received:", event.data);
        };
    };

    // Monitor connection state
    pc.onconnectionstatechange = function () {
        console.log("Connection state:", pc.connectionState);
    };

    pc.oniceconnectionstatechange = function () {
        console.log("ICE connection state:", pc.iceConnectionState);
    };

    console.log("WebRTC helper setup complete!");
};

// Helper function to get DataChannel
window.getWebRTCDataChannel = function () {
    return window._webrtc_datachannel;
};
