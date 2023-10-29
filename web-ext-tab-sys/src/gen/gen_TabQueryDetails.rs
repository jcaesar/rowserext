#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabQueryDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabQueryDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub type TabQueryDetails;
}
impl TabQueryDetails {
    #[doc = "Construct a new `TabQueryDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn active(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("active"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `audible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn audible(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("audible"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `currentWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn current_window(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("currentWindow"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `highlighted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn highlighted(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("highlighted"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `index` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn index(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("index"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `lastFocusedWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn last_focused_window(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("lastFocusedWindow"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `muted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn muted(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("muted"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pinned` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("pinned"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "TabStatus")]
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`, `TabStatus`*"]
    pub fn status(&mut self, val: Option<TabStatus>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("status"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn title(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("title"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn url(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("url"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `windowId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`*"]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("windowId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TabQueryDetails {
    fn default() -> Self {
        Self::new()
    }
}
