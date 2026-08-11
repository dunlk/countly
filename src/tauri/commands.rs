use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        js_namespace = ["window", "__TAURI_INTERNALS__"],
        js_name = invoke
    )]
    fn invoke_js(command: &str, args: JsValue) -> Promise;
}

pub fn invoke(command: &str, args: JsValue) -> Promise {
    invoke_js(command, args)
}
