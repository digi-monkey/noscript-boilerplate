use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use whatlang::{detect_script, Script::Mandarin};

#[wasm_bindgen]
pub fn pre_validate(){
    // do nothing..
}

#[wasm_bindgen]
pub fn is_valid_event(event: JsValue) -> bool {
    if let Some(obj) = event.dyn_ref::<js_sys::Object>() {
        if let Ok(content) = Reflect::get(obj, &JsValue::from_str("content")) {
            if let Some(content) = content.as_string() {
                let info = detect_script(content.as_str()).unwrap();
                if info.eq(&Mandarin){
                    return true;
                }
           }
        }
    }
    false
}
