use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{RtcPeerConnection, RtcConfiguration, RtcIceServer, RtcIceCandidateInit, RtcSessionDescriptionInit, RtcSdpType, RtcDataChannel};
use yew::Callback;
use shared::SignalMessage;
use std::rc::Rc;
use std::cell::RefCell;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setupWebRTCHelper)]
    fn setup_webrtc_helper(pc: &RtcPeerConnection);
    
    #[wasm_bindgen(js_name = getWebRTCDataChannel)]
    pub fn get_webrtc_datachannel() -> Option<RtcDataChannel>;
}

pub struct WebRTCService {
    pub pc: RtcPeerConnection,
}

impl WebRTCService {
    pub fn new(on_ice: Callback<String>, on_data_channel: Option<Callback<RtcDataChannel>>) -> Result<Self, JsValue> {
        let mut rtc_config = RtcConfiguration::new();
        let ice_server = RtcIceServer::new();
        ice_server.set_urls(&JsValue::from_str("stun:stun.l.google.com:19302"));
        let ice_servers = js_sys::Array::new();
        ice_servers.push(&ice_server);
        rtc_config.set_ice_servers(&ice_servers);

        let pc = RtcPeerConnection::new_with_configuration(&rtc_config)?;

        // Handle ICE candidates
        let on_ice_clone = on_ice.clone();
        let onicecandidate = Closure::wrap(Box::new(move |e: web_sys::RtcPeerConnectionIceEvent| {
            if let Some(candidate) = e.candidate() {
                if let Ok(json) = js_sys::JSON::stringify(&candidate.to_json()) {
                    if let Some(candidate_str) = json.as_string() {
                        on_ice_clone.emit(candidate_str);
                    }
                }
            }
        }) as Box<dyn FnMut(web_sys::RtcPeerConnectionIceEvent)>);
        pc.set_onicecandidate(Some(onicecandidate.as_ref().unchecked_ref()));
        onicecandidate.forget();

        // Handle Data Channel using JavaScript helper with polling
        if let Some(_on_dc) = on_data_channel {
            web_sys::console::log_1(&"Using JavaScript helper for DataChannel (polling mode)...".into());
            
            // Just set up the JavaScript helper without callback
            setup_webrtc_helper(&pc);
            
            web_sys::console::log_1(&"JavaScript helper setup complete! DataChannel will be polled.".into());
        }

        // Log connection state changes
        let onconnectionstatechange = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            // Note: we can't access pc here without cloning, so we'll log from main.rs instead
        }) as Box<dyn FnMut(web_sys::Event)>);
        pc.set_onconnectionstatechange(Some(onconnectionstatechange.as_ref().unchecked_ref()));
        onconnectionstatechange.forget();

        // Log ICE connection state changes
        let oniceconnectionstatechange = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            // Note: we can't access pc here without cloning, so we'll log from main.rs instead
        }) as Box<dyn FnMut(web_sys::Event)>);
        pc.set_oniceconnectionstatechange(Some(oniceconnectionstatechange.as_ref().unchecked_ref()));
        oniceconnectionstatechange.forget();

        Ok(Self { pc })
    }

    pub async fn create_offer(&self) -> Result<String, JsValue> {
        let offer = wasm_bindgen_futures::JsFuture::from(self.pc.create_offer()).await?;
        let offer_sdp = offer.unchecked_into::<RtcSessionDescriptionInit>();
        wasm_bindgen_futures::JsFuture::from(self.pc.set_local_description(&offer_sdp)).await?;
        let sdp = js_sys::Reflect::get(&offer_sdp, &JsValue::from_str("sdp"))?.as_string().unwrap_or_default();
        Ok(sdp)
    }

    pub async fn create_answer(&self) -> Result<String, JsValue> {
        let answer = wasm_bindgen_futures::JsFuture::from(self.pc.create_answer()).await?;
        let answer_sdp = answer.unchecked_into::<RtcSessionDescriptionInit>();
        wasm_bindgen_futures::JsFuture::from(self.pc.set_local_description(&answer_sdp)).await?;
        let sdp = js_sys::Reflect::get(&answer_sdp, &JsValue::from_str("sdp"))?.as_string().unwrap_or_default();
        Ok(sdp)
    }

    pub async fn set_remote_description(&self, sdp: &str, type_: RtcSdpType) -> Result<(), JsValue> {
        let mut description = RtcSessionDescriptionInit::new(type_);
        description.set_sdp(sdp);
        wasm_bindgen_futures::JsFuture::from(self.pc.set_remote_description(&description)).await?;
        Ok(())
    }

    pub async fn add_ice_candidate(&self, candidate_json: &str) -> Result<(), JsValue> {
        let candidate_obj = js_sys::JSON::parse(candidate_json)?;
        let candidate = RtcIceCandidateInit::unchecked_from_js(candidate_obj);
        wasm_bindgen_futures::JsFuture::from(self.pc.add_ice_candidate_with_opt_rtc_ice_candidate_init(Some(&candidate))).await?;
        Ok(())
    }
}
