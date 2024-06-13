mod utils;

use std::thread::sleep;
use std::time;
use wasm_bindgen::prelude::*;
use web_sys::{console, MessageEvent, Worker};
use web_sys::js_sys::parse_float;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-test!");
}

#[wasm_bindgen]
pub fn start_worker() -> Result<(), JsValue> {
    // Define the URL of the worker script
    let worker_js = "../worker.js";

    // Create a new Worker
    let worker = Worker::new(worker_js)?;

    // Post a message to the worker
    worker.post_message(&JsValue::from_str("Hello from main thread!"))?;

    // Handle messages from the worker
    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        web_sys::console::log_1(&JsValue::from_str(&format!("Received from worker: {:?}", event.data().as_string())));
    }) as Box<dyn FnMut(_)>);

    worker.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    Ok(())
}