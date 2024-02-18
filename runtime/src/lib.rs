use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use types::Event;

#[wasm_bindgen]
extern "C" {
    fn selfEvent() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    fn getEventById(id: String) -> Promise;
}

pub fn get_self_event() -> Result<Event, JsValue> {
    let js_event: JsValue = selfEvent();
    let rust_event: Event = serde_wasm_bindgen::from_value(js_event)?;
    Ok(rust_event)
}

pub async fn get_event_by_id(id: String) -> Result<Event, JsValue> {
    let promise = getEventById(id);
    let js_event = JsFuture::from(promise).await?;
    let rust_event: Event = serde_wasm_bindgen::from_value(js_event)?;
    Ok(rust_event)
}
