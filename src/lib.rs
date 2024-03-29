use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say_alert(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}