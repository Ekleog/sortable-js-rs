//! This crate provides rusty bindings to SortableJS.
//!
//! The documentation mostly lives with [the official SortableJS documentation](https://github.com/SortableJS/Sortable).
//!
//! Just adding this crate as a dependency should be enough to get everything working when using [trunk](https://trunkrs.dev/). Just be careful to keep alive the return value of `Sortable::apply`, or you will get JavaScript exceptions.
//!
//! You can find example usage of SortableJS from a pure-Rust WASM application
//! in the `examples/` directory.

use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/sortable.esm.js")]
    extern "C" {
        #[wasm_bindgen(extends = js_sys::Object)]
        pub type Sortable;

        #[wasm_bindgen(constructor)]
        pub fn new(elt: &web_sys::Element, opts: &js_sys::Object) -> Sortable;

        #[wasm_bindgen(method)]
        pub fn destroy(item: &Sortable);
    }
}

/// An event raised by one of the Sortable callbacks. See [the official documentation](https://github.com/SortableJS/Sortable#event-object-demo) for details about the fields.
///
/// `raw_event` contains the raw JS event, should additional non-documented
/// fields be needed.
#[derive(Clone, Debug)]
pub struct Event {
    pub raw_event: js_sys::Object,
    pub to: web_sys::HtmlElement,
    pub from: web_sys::HtmlElement,
    pub item: web_sys::HtmlElement,
    pub clone: web_sys::HtmlElement,
    pub old_index: Option<usize>,
    pub new_index: Option<usize>,
    pub old_draggable_index: Option<usize>,
    pub new_draggable_index: Option<usize>,
    // TODO: pullMode
}

impl Event {
    fn from_raw_event(raw_event: js_sys::Object) -> Event {
        macro_rules! get {
            ($field:expr) => {
                js_sys::Reflect::get(&raw_event, &JsValue::from_str($field))
                    .expect("failed retrieving field from raw event")
                    .dyn_into()
                    .expect("failed casting field of raw event to proper type")
            };
        }
        macro_rules! get_optint {
            ($field:expr) => {
                js_sys::Reflect::get(&raw_event, &JsValue::from_str($field))
                    .ok()
                    .map(|evt| {
                        let float = evt
                            .as_f64()
                            .expect("failed casting field of raw event to proper type");
                        let int = float as usize;
                        assert!(
                            (int as f64 - float).abs() < 0.1,
                            "received index that is not an integer: {}",
                            float
                        );
                        int
                    })
            };
        }
        Event {
            to: get!("to"),
            from: get!("from"),
            item: get!("item"),
            clone: get!("clone"),
            old_index: get_optint!("oldIndex"),
            new_index: get_optint!("newIndex"),
            old_draggable_index: get_optint!("oldDraggableIndex"),
            new_draggable_index: get_optint!("newDraggableIndex"),
            raw_event,
        }
    }
}

#[repr(usize)]
enum CallbackId {
    Choose,
    Unchoose,
    Start,
    End,
    Add,
    Update,
    Sort,
    Remove,
    Filter,
    Clone,
    Change,
    Spill,
    _Total,
}

/// See https://github.com/SortableJS/Sortable for more documentation about available options
pub struct Options {
    options: js_sys::Object,
    callbacks: [Option<Rc<Closure<dyn FnMut(js_sys::Object)>>>; CallbackId::_Total as usize],
}

macro_rules! option {
    ( $setter:ident, $jsname:expr, $typ:ty, $builder:ident ) => {
        pub fn $setter(&mut self, value: $typ) -> &mut Options {
            let res = js_sys::Reflect::set(
                &self.options,
                &JsValue::from_str($jsname),
                &JsValue::$builder(value),
            )
            .expect("setting property on object failed");
            assert!(res, "failed setting property on object");
            self
        }
    };
}

macro_rules! callback {
    ( $setter:ident, $jsname:expr, $id:ident ) => {
        pub fn $setter(&mut self, mut cb: impl 'static + FnMut(Event)) -> &Options {
            let cb = Closure::new(move |e: js_sys::Object| cb(Event::from_raw_event(e)));
            let res = js_sys::Reflect::set(&self.options, &JsValue::from_str($jsname), cb.as_ref())
                .expect("setting callback on object failed");
            assert!(res, "failed setting callback on object");
            self.callbacks[CallbackId::$id as usize] = Some(Rc::new(cb));
            self
        }
    };
}

impl Options {
    /// Create a builder for `Sortable`
    ///
    /// This builder allows for configuration options to be set. Once the
    /// desired configuration is done, use `apply` to apply it to a list.
    pub fn new() -> Options {
        Options {
            options: js_sys::Object::new(),
            callbacks: std::array::from_fn(|_| None),
        }
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
    option!(
        inverted_swap_threshold,
        "invertedSwapThreshold",
        f64,
        from_f64
    );
    option!(direction, "direction", &str, from_str);

    option!(force_fallback, "forceFallback", bool, from_bool);

    option!(fallback_class, "fallbackClass", &str, from_str);
    option!(fallback_on_body, "fallbackOnBody", bool, from_bool);
    option!(fallback_tolerance, "fallbackTolerance", f64, from_f64);

    option!(dragover_bubble, "dragoverBubble", bool, from_bool);
    option!(remove_clone_on_hide, "removeCloneOnHide", bool, from_bool);
    option!(
        empty_insert_threshold,
        "emptyInsertThreshold",
        f64,
        from_f64
    );

    callback!(on_choose, "onChoose", Choose);
    callback!(on_unchoose, "onUnchoose", Unchoose);
    callback!(on_start, "onStart", Start);
    callback!(on_end, "onEnd", End);
    callback!(on_add, "onAdd", Add);
    callback!(on_update, "onUpdate", Update);
    callback!(on_sort, "onSort", Sort);
    callback!(on_remove, "onRemove", Remove);
    callback!(on_filter, "onFilter", Filter);
    callback!(on_clone, "onClone", Clone);
    callback!(on_change, "onChange", Change);

    // TODO: onMove

    // RevertOnSpill / RemoveOnSpill plugins
    option!(revert_on_spill, "revertOnSpill", bool, from_bool);
    option!(remove_on_spill, "removeOnSpill", bool, from_bool);
    callback!(on_spill, "onSpill", Spill);

    /// Recover the javascript options that are being built in this object
    ///
    /// Note that you can set options on this object through `js_sys::Reflect`.
    /// This allows setting options that are not planned for by
    /// `sortable-js-rs`.
    pub fn options(&self) -> &js_sys::Object {
        &self.options
    }

    /// Apply the current configuration as a `Sortable` instance on `elt`
    ///
    /// Do not forget to keep the return value of this function alive for as
    /// long as you want the callbacks to be callable, as JS code will error out
    /// should an event happen after it was dropped.
    pub fn apply(&self, elt: &web_sys::Element) -> Sortable {
        let sortable = js::Sortable::new(elt, &self.options);
        let object_ref: &js_sys::Object = sortable.as_ref();
        let raw_object = object_ref.clone();
        Sortable {
            raw_object,
            sortable,
            _callbacks: self.callbacks.clone(),
        }
    }
}

/// Data related to the Sortable instance
///
/// When it is dropped, the list is made non-sortable again. This is required
/// because callbacks could be called otherwise. If it is a problem for you, you
/// can leak it, but be aware of the fact that it is a leak.
pub struct Sortable {
    /// Raw Sortable JS object, should this crate not expose the necessary
    /// methods
    pub raw_object: js_sys::Object,

    /// raw_object but with the proper type
    sortable: js::Sortable,

    /// Keep the callbacks alive
    _callbacks: [Option<Rc<Closure<dyn FnMut(js_sys::Object)>>>; CallbackId::_Total as usize],
}

impl Drop for Sortable {
    fn drop(&mut self) {
        self.sortable.destroy();
    }
}
