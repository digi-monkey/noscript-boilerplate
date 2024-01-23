use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pre_validate() {
    // do nothing..
}

#[wasm_bindgen]
pub fn is_valid_event(event: JsValue) -> bool {
    if let Some(obj) = event.dyn_ref::<js_sys::Object>() {
        if let Ok(content) = Reflect::get(obj, &JsValue::from_str("content")) {
            if let Some(content) = content.as_string() {
                if ends_with_media_extension(&content) {
                    return true;
                }
            }
        }
    }
    false
}

fn ends_with_media_extension(url: &str) -> bool {
    let media_extensions = ["jpg", "jpeg", "png", "gif", "mp4"];

    // Check if the URL ends with any of the specified media file extensions
    for &extension in &media_extensions {
        if url.to_lowercase().ends_with(extension) {
            return true;
        }
    }

    false
}
