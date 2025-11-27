use shared::SignalMessage;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
use yew::Callback;

pub struct SignalingService {
    ws: WebSocket,
}

impl SignalingService {
    pub fn new(url: &str, on_message: Callback<SignalMessage>, on_open: Callback<()>) -> Result<Self, JsValue> {
        let ws = WebSocket::new(url)?;
        
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let txt: String = txt.into();
                if let Ok(msg) = serde_json::from_str::<SignalMessage>(&txt) {
                    on_message.emit(msg);
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onopen_callback = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"WebSocket Connected!".into());
            on_open.emit(());
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |e: web_sys::Event| {
            web_sys::console::error_1(&"WebSocket Error!".into());
            web_sys::console::error_1(&e);
        }) as Box<dyn FnMut(web_sys::Event)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let onclose_callback = Closure::wrap(Box::new(move |e: web_sys::CloseEvent| {
            web_sys::console::warn_1(&format!("WebSocket Closed: {} (Clean: {})", e.reason(), e.was_clean()).into());
        }) as Box<dyn FnMut(web_sys::CloseEvent)>);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
        
        Ok(Self { ws })
    }

    pub fn send(&self, msg: SignalMessage) -> Result<(), JsValue> {
        web_sys::console::log_1(&format!("Sending signal: {:?}", msg).into());
        let txt = serde_json::to_string(&msg).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.ws.send_with_str(&txt)
    }
}
