use web_sys::{File, FileReader};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use js_sys::Uint8Array;

const CHUNK_SIZE: usize = 16384; // 16KB chunks

pub async fn read_file_as_bytes(file: File) -> Result<Vec<u8>, String> {
    let file_reader = FileReader::new().map_err(|e| format!("{:?}", e))?;
    
    let (sender, receiver) = futures::channel::oneshot::channel();
    let sender = std::rc::Rc::new(std::cell::RefCell::new(Some(sender)));
    
    let onload = {
        let file_reader = file_reader.clone();
        let sender = sender.clone();
        Closure::wrap(Box::new(move || {
            if let Ok(result) = file_reader.result() {
                let array = Uint8Array::new(&result);
                let bytes = array.to_vec();
                if let Some(sender) = sender.borrow_mut().take() {
                    let _ = sender.send(Ok(bytes));
                }
            }
        }) as Box<dyn FnMut()>)
    };
    
    let onerror = {
        let sender = sender.clone();
        Closure::wrap(Box::new(move || {
            if let Some(sender) = sender.borrow_mut().take() {
                let _ = sender.send(Err("Failed to read file".to_string()));
            }
        }) as Box<dyn FnMut()>)
    };
    
    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    
    file_reader.read_as_array_buffer(&file).map_err(|e| format!("{:?}", e))?;
    
    onload.forget();
    onerror.forget();
    
    receiver.await.map_err(|_| "Channel closed".to_string())?
}

pub fn chunk_data(data: &[u8]) -> Vec<Vec<u8>> {
    data.chunks(CHUNK_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect()
}
