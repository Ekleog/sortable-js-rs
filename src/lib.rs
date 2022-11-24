use wasm_bindgen::JsValue;

mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/Sortable.min.js")]
    extern "C" {
        pub type Sortable;

        #[wasm_bindgen(constructor)]
        pub fn new(elt: &web_sys::Element, opts: &js_sys::Map) -> Sortable;
    }
}

pub struct Options(js_sys::Map);

impl Options {
    pub fn new() -> Options {
        Options(js_sys::Map::new())
    }

    pub fn set_group(&self, group: &str) -> &Options {
        self.0.set(&JsValue::from_str("group"), &JsValue::from_str(group));
        self
    }

    pub fn apply(&self, elt: &web_sys::Element) {
        js::Sortable::new(elt, &self.0);
    }
}
