use wasm_bindgen::prelude::*;

fn main() {
    web_sys::console::log_1(&JsValue::from_str("Wasm init complete"));
    let the_sortable = sortable_js::Options::new()
        .animation_ms(150.)
        .on_update(|e| {
            web_sys::console::log_2(
                &JsValue::from_str(&format!("got event: rust is {:?}, js is ", e)),
                &e.raw_event,
            )
        })
        .apply(
            &web_sys::window()
                .expect("had no window")
                .document()
                .expect("had no document")
                .get_element_by_id("list")
                .expect("could not find #list"),
        );
    Box::leak(Box::new(the_sortable));
    web_sys::console::log_1(&JsValue::from_str("Wasm execution finished"));
}
