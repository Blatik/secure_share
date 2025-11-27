mod file_transfer;
mod storage;
mod crypto;

use storage::StorageService;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{InputEvent, Event, Url, Blob};
use std::rc::Rc;
use std::cell::RefCell;
use crate::crypto::CryptoService;

use yew::prelude::*;
use yew::functional::*;
use yew::TargetCast;

#[function_component(App)]
pub fn app() -> Html {
    let mode = use_state(|| "home".to_string()); // home, sender, receiver
    let session_id = use_state(|| String::new());

    // Auto-detect receiver mode from URL params
    {
        let mode = mode.clone();
        use_effect_with((), move |_| {
            // Initialize Telegram Web App
            if let Some(window) = web_sys::window() {
                if let Ok(telegram) = js_sys::Reflect::get(&window, &JsValue::from_str("Telegram")) {
                    if let Ok(web_app) = js_sys::Reflect::get(&telegram, &JsValue::from_str("WebApp")) {
                        if !web_app.is_undefined() {
                            let _ = js_sys::Reflect::get(&web_app, &JsValue::from_str("ready"))
                                .and_then(|ready_fn| {
                                    let ready_fn = ready_fn.dyn_into::<js_sys::Function>()?;
                                    ready_fn.call0(&web_app)
                                });
                            
                            let _ = js_sys::Reflect::get(&web_app, &JsValue::from_str("expand"))
                                .and_then(|expand_fn| {
                                    let expand_fn = expand_fn.dyn_into::<js_sys::Function>()?;
                                    expand_fn.call0(&web_app)
                                });
                        }
                    }
                }
            }

            let window = web_sys::window().unwrap();
            let search = window.location().search().unwrap_or_default();
            let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
            
            // Check standard params
            if params.get("id").is_some() && params.get("key").is_some() {
                mode.set("receiver".to_string());
            }
            // Check Telegram startapp param
            else if let Some(startapp) = params.get("tgWebAppStartParam").or(params.get("startapp")) {
                if startapp.contains('_') {
                    mode.set("receiver".to_string());
                }
            }
            || ()
        });
    }

    let on_send_click = {
        let mode = mode.clone();
        Callback::from(move |_| {
            mode.set("sender".to_string());
        })
    };

    let on_receive_click = {
        let mode = mode.clone();
        Callback::from(move |_| {
            mode.set("receiver".to_string());
        })
    };

    html! {
        <div class="container">
            <NavBar on_change_mode={
                let mode = mode.clone();
                Callback::from(move |new_mode: String| mode.set(new_mode))
            } />
            
            <AdBanner position="top" />
            
            <h1>
                <svg class="icon" width="24" height="24" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                {"SecureShare"}
            </h1>
            
            if *mode == "home" {
                <div class="home-actions">
                    <p class="hero-text">{"Secure, end-to-end encrypted file sharing directly in your browser. Upload files up to 100MB. Files are stored for 10 minutes. No registration, no logs, just privacy."}</p>
                    <div class="action-buttons">
                        <button onclick={on_send_click} class="primary-btn">
                            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path></svg>
                            {"Send File"}
                        </button>
                        <button onclick={on_receive_click} class="secondary-btn">
                            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
                            {"Receive File"}
                        </button>
                    </div>
                    
                    <div class="how-it-works">
                        <h3>{"How it Works"}</h3>
                        <div class="steps">
                            <div class="step">
                                <span class="step-num">{"1"}</span>
                                <h4>{"Select File"}</h4>
                                <p>{"Choose any file. It is encrypted instantly in your browser using ChaCha20-Poly1305."}</p>
                            </div>
                            <div class="step">
                                <span class="step-num">{"2"}</span>
                                <h4>{"Get Link"}</h4>
                                <p>{"We generate a unique link with the decryption key. The server never sees the key."}</p>
                            </div>
                            <div class="step">
                                <span class="step-num">{"3"}</span>
                                <h4>{"Share Securely"}</h4>
                                <p>{"Send the link to your recipient. They can download and decrypt the file instantly."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            } else if *mode == "sender" {
                <SenderView />
            } else if *mode == "receiver" {
                <ReceiverView />
            }
            
            <AdBanner position="bottom" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    pub on_change_mode: Callback<String>,
}

#[function_component(NavBar)]
pub fn nav_bar(props: &NavBarProps) -> Html {
    let on_click = |mode: &str| {
        let mode = mode.to_string();
        let cb = props.on_change_mode.clone();
        Callback::from(move |_| cb.emit(mode.clone()))
    };

    html! {
        <nav class="nav-bar">
            <div class="nav-links">
                <a onclick={on_click("home")} class="nav-link">{"Home"}</a>
                <a onclick={on_click("sender")} class="nav-link">{"Send"}</a>
                <a onclick={on_click("receiver")} class="nav-link">{"Receive"}</a>
            </div>
            <div class="nav-external">
                <a href="https://blatik.github.io/rustdev-network/" target="_blank" class="nav-link external">
                    {"RustDev Network"}
                    <svg class="icon-small" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path></svg>
                </a>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct AdBannerProps {
    pub position: String,
}

#[function_component(AdBanner)]
pub fn ad_banner(props: &AdBannerProps) -> Html {
    let (title, button_text, link) = if props.position == "top" {
        ("RustDev Network", "Відвідати →", "https://blatik.github.io/rustdev-network/")
    } else {
        ("SecureShare в Telegram", "Відкрити бота →", "https://t.me/b_secure_share_bot")
    };

    html! {
        <div class={format!("ad-banner ad-{}", props.position)}>
            <div class="ad-content">
                <span class="ad-title">{title}</span>
                <a href={link} target="_blank" rel="noopener noreferrer" class="ad-button">
                    {button_text}
                </a>
            </div>
        </div>
    }
}

#[function_component(SenderView)]
pub fn sender_view() -> Html {
    let status = use_state(|| "Select a file to upload".to_string());
    let file_info = use_state(|| None::<(String, String)>); // (id, key)
    let copy_status = use_state(|| "Copy Link".to_string()); // Moved to top level

    let on_file_change = {
        let status = status.clone();
        let file_info = file_info.clone();
        
        Callback::from(move |e: Event| {
            // ... (same as before)
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let status = status.clone();
                    let file_info = file_info.clone();
                    let file_name = file.name();
                    
                    status.set(format!("Reading {}...", file_name));
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        // 1. Read file
                        match file_transfer::read_file_as_bytes(file.clone()).await {
                            Ok(data) => {
                                // 2. Generate Key
                                let key = CryptoService::generate_key();
                                status.set("Encrypting...".to_string());

                                // 3. Encrypt
                                match CryptoService::encrypt_with_key(&data, &key) {
                                    Ok(encrypted_data) => {
                                        status.set("Uploading...".to_string());
                                        
                                        match StorageService::upload_data(encrypted_data, file_name, file.type_()).await {
                                            Ok(id) => {
                                                file_info.set(Some((id, key)));
                                                status.set("File uploaded & encrypted! ✅".to_string());
                                            }
                                            Err(e) => status.set(format!("Upload error: {}", e)),
                                        }
                                    }
                                    Err(e) => status.set(format!("Encryption error: {}", e)),
                                }
                            }
                            Err(e) => status.set(format!("Read error: {}", e)),
                        }
                    });
                }
            }
        })
    };

    let on_copy = {
        let file_info = file_info.clone();
        let copy_status = copy_status.clone();
        Callback::from(move |_| {
            if let Some((id, key)) = &*file_info {
                 let window = web_sys::window().unwrap();
                 let location = window.location();
                 let base_url = format!("{}{}", 
                     location.origin().unwrap(),
                     location.pathname().unwrap()
                 );
                 let link = format!("{}?id={}&key={}", base_url, id, urlencoding::encode(key));
                 // Use write_text directly
                 let _ = window.navigator()
                     .clipboard()
                     .write_text(&link);
                 copy_status.set("Copied! 📋".to_string());
                 
                 let copy_status = copy_status.clone();
                 gloo_timers::callback::Timeout::new(2000, move || {
                     copy_status.set("Copy Link".to_string());
                 }).forget();
            }
        })
    };

    // Generate link outside html! macro
    let link_opt = file_info.as_ref().map(|(id, key)| {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let base_url = format!("{}{}", 
            location.origin().unwrap(),
            location.pathname().unwrap()
        );
        format!("{}?id={}&key={}", base_url, id, urlencoding::encode(key))
    });

    html! {
        <div class="sender-view">
            <h2>{"Sender Mode"}</h2>
            
            if let Some(link) = link_opt {
                <div class="success-box">
                    <p>{"File Encrypted & Uploaded!"}</p>
                    
                    <div class="link-group">
                        <p>{"Web Link:"}</p>
                        <div class="link-box">
                            <a href={link.clone()} target="_blank">
                                {link.clone()}
                            </a>
                        </div>
                        <button onclick={on_copy} class="copy-btn">
                            {&**copy_status}
                        </button>
                    </div>

                    if let Some((id, key)) = &*file_info {
                        <div class="link-group" style="margin-top: 1rem;">
                            <p>{"Telegram Link (Open in App):"}</p>
                            <div class="link-box">
                                {format!("https://t.me/b_secure_share_bot?start={}_{}", id, key)}
                            </div>
                            <button onclick={
                                let id = id.clone();
                                let key = key.clone();
                                Callback::from(move |_| {
                                    let tg_link = format!("https://t.me/b_secure_share_bot?start={}_{}", id, key);
                                    let _ = web_sys::window().unwrap().navigator().clipboard().write_text(&tg_link);
                                })
                            } class="copy-btn">
                                {"Copy Telegram Link 📋"}
                            </button>
                        </div>
                    }

                    <p class="warning">{"Note: The key is in the link. Don't share it publicly!"}</p>
                </div>
            }
            
            <input 
                type="file" 
                id="file-input"
                style="margin-top: 1rem;"
                onchange={on_file_change}
            />
            
            <div class="status-badge">{&**status}</div>
        </div>
    }
}

#[function_component(ReceiverView)]
pub fn receiver_view() -> Html {
    let file_id_input = use_state(|| String::new());
    let key_input = use_state(|| String::new());
    let status = use_state(|| "Enter File ID and Key".to_string());
    
    // Check URL params on load
    {
        let file_id_input = file_id_input.clone();
        let key_input = key_input.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let search = window.location().search().unwrap_or_default();
            let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
            
            // 1. Check standard params
            if let Some(id) = params.get("id") {
                file_id_input.set(id);
            }
            if let Some(key) = params.get("key") {
                key_input.set(key);
            }

            // 2. Check Telegram startapp param (format: id_key)
            // This needs to be handled carefully. If we redirect, the effect will run again.
            // For now, we'll just parse it if present and set the state.
            // A redirect would typically happen at a higher level (e.g., App component)
            // to ensure the URL is canonical before the component renders.
            if let Some(startapp) = params.get("tgWebAppStartParam").or(params.get("startapp")) {
                if let Some((id_part, key_part)) = startapp.split_once('_') {
                    // Only set if standard params weren't already set, or if startapp is more specific
                    if file_id_input.is_empty() && key_input.is_empty() {
                        file_id_input.set(id_part.to_string());
                        key_input.set(key_part.to_string());
                    }
                }
            }
            || ()
        });
    }

    let on_download = {
        let file_id_input = file_id_input.clone();
        let key_input = key_input.clone();
        let status = status.clone();

        Callback::from(move |_| {
            let id = (*file_id_input).clone();
            let key = (*key_input).clone();
            
            if id.trim().is_empty() || key.trim().is_empty() {
                status.set("Please enter File ID and Key".to_string());
                return;
            }
            let status = status.clone();

            wasm_bindgen_futures::spawn_local(async move {
                status.set("Downloading encrypted file...".to_string());
                match StorageService::download_file(&id).await {
                    Ok((encrypted_data, filename, mime_type)) => {
                        status.set("Decrypting...".to_string());
                        
                        match CryptoService::decrypt_with_key(&encrypted_data, &key) {
                            Ok(decrypted_data) => {
                                status.set("File decrypted! Saving...".to_string());
                                
                                // Trigger Download
                                if let Ok(array) = js_sys::Uint8Array::from(&decrypted_data[..]).dyn_into::<js_sys::Object>() {
                                    let array_sequence = js_sys::Array::new();
                                    array_sequence.push(&array);
                                    
                                    let mut blob_options = web_sys::BlobPropertyBag::new();
                                    blob_options.set_type(&mime_type);
                                    
                                    if let Ok(blob) = Blob::new_with_u8_array_sequence_and_options(&array_sequence, &blob_options) {
                                        if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                                            let document = web_sys::window().unwrap().document().unwrap();
                                            if let Ok(a) = document.create_element("a") {
                                                let _ = a.set_attribute("href", &url);
                                                let _ = a.set_attribute("download", &filename);
                                                let _ = a.set_attribute("style", "display: none");
                                                if let Ok(body) = document.body().ok_or("no body") {
                                                    let _ = body.append_child(&a);
                                                    let html_element = a.unchecked_into::<web_sys::HtmlElement>();
                                                    html_element.click();
                                                    let _ = body.remove_child(&html_element);
                                                    let _ = Url::revoke_object_url(&url);
                                                    status.set("Download complete! ✅".to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => status.set(format!("Decryption error: {}", e)),
                        }
                    }
                    Err(e) => {
                        status.set(format!("Download error: {}", e));
                    }
                }
            });
        })
    };

    html! {
        <div class="receiver-view">
            <h2>{"Receiver Mode"}</h2>
            <input 
                type="text" 
                placeholder="File ID" 
                value={(*file_id_input).clone()} 
                oninput={Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    file_id_input.set(input.value());
                })}
            />
            <input 
                type="text" 
                placeholder="Decryption Key" 
                value={(*key_input).clone()} 
                oninput={Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    key_input.set(input.value());
                })}
            />
            <button onclick={on_download}>{"Download & Decrypt"}</button>
            <div class="status-badge">{&**status}</div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
