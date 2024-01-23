use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pre_validate(){
    // do nothing..
}

#[wasm_bindgen]
pub fn is_valid_event(event: JsValue) -> bool {
    if let Some(obj) = event.dyn_ref::<js_sys::Object>() {
        if let Ok(content) = Reflect::get(obj, &JsValue::from_str("content")) {
            if let Some(content) = content.as_string() {
                let word_count = count_words(&content);
                if word_count >= 600{
                    return true;
                }
           }
        }
    }
    false
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
