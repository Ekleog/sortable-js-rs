use wasm_bindgen::prelude::*;

fn main() {
    web_sys::console::log_1(&JsValue::from_str("Wasm init complete"));
    sortable_js::Options::new()
        .animation_ms(150.)
        .apply(
            &web_sys::window()
                .expect("had no window")
                .document()
                .expect("had no document")
                .get_element_by_id("list")
                .expect("could not find #list")
        );
    web_sys::console::log_1(&JsValue::from_str("Wasm execution finished"));
}
