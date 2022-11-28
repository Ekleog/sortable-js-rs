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

    option!(group, "group", &str, from_str);
    option!(sort, "sort", bool, from_bool);
    option!(delay, "delay", f64, from_f64);
    option!(delay_on_touch_only, "delayOnTouchOnly", bool, from_bool);
    option!(touch_start_threshold, "touchStartThreshold", f64, from_f64);
    option!(disabled, "disabled", bool, from_bool);
    // TODO: consider supporting the Store option
    option!(animation_ms, "animation", f64, from_f64);
    option!(easing, "easing", &str, from_str);
    option!(handle, "handle", &str, from_str);
    option!(filter, "filter", &str, from_str);
    option!(prevent_on_filter, "preventOnFilter", bool, from_bool);
    option!(draggable, "draggable", &str, from_str);

    option!(data_id_attr, "dataIdAttr", &str, from_str);

    option!(ghost_class, "ghostClass", &str, from_str);
    option!(chosen_class, "chosenClass", &str, from_str);
    option!(drag_class, "dragClass", &str, from_str);

    option!(swap_threshold, "swapThreshold", f64, from_f64);
    option!(invert_swap, "invertSwap", bool, from_bool);
    option!(inverted_swap_threshold, "invertedSwapThreshold", f64, from_f64);
    option!(direction, "direction", &str, from_str);

    option!(force_fallback, "forceFallback", bool, from_bool);

    option!(fallback_class, "fallbackClass", &str, from_str);
    option!(fallback_on_body, "fallbackOnBody", bool, from_bool);
    option!(fallback_tolerance, "fallbackTolerance", f64, from_f64);

    option!(dragover_bubble, "dragoverBubble", bool, from_bool);
    option!(remove_clone_on_hide, "removeCloneOnHide", bool, from_bool);
    option!(empty_insert_threshold, "emptyInsertThreshold", f64, from_f64);

    // TODO: all the callbacks

    /// Recover the javascript options that are being built in this object.
    ///
    /// Note that you can set options on this object through `js_sys::Reflect`.
    /// This allows setting options that are not planned for by `sortable-js-rs`.
    pub fn options(&self) -> &js_sys::Object {
        &self.0
    }

    pub fn apply(&self, elt: &web_sys::Element) {
        js::Sortable::new(elt, &self.0);
    }
}
