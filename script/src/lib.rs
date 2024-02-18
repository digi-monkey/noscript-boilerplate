extern crate runtime;

use runtime::get_event_by_id;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn build_feed(id: String) -> JsValue {
    let event = get_event_by_id(id).await.unwrap();
    serde_wasm_bindgen::to_value(&event).unwrap()
}
