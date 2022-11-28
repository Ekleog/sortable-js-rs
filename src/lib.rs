use wasm_bindgen::JsValue;

mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/sortable.esm.js")]
    extern "C" {
        pub type Sortable;

        #[wasm_bindgen(constructor)]
        pub fn new(elt: &web_sys::Element, opts: &js_sys::Object) -> Sortable;
    }
}

/// See https://github.com/SortableJS/Sortable for more documentation about available options
pub struct Options(js_sys::Object);

macro_rules! option {
    ( $setter:ident, $jsname:expr, $typ:ty, $builder:ident ) => {
        pub fn $setter(&self, value: $typ) -> &Options {
            let res = js_sys::Reflect::set(
                &self.0,
                &JsValue::from_str($jsname),
                &JsValue::$builder(value)
            ).expect("setting property on object failed");
            assert!(res, "failed setting property on object");
            self
        }
    }
}

impl Options {
    pub fn new() -> Options {
        Options(js_sys::Object::new())
    }

    option!(set_group, "group", &str, from_str);
    option!(set_sort, "sort", bool, from_bool);
    option!(set_delay, "delay", f64, from_f64);
    option!(set_disabled, "disabled", bool, from_bool);
    option!(set_handle, "handle", &str, from_str);

    pub fn apply(&self, elt: &web_sys::Element) {
        js::Sortable::new(elt, &self.0);
    }
}
