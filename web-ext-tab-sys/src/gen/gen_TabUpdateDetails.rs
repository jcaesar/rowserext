#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabUpdateDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabUpdateDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
    pub type TabUpdateDetails;
}
impl TabUpdateDetails {
    #[doc = "Construct a new `TabUpdateDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
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
    #[doc = "Change the `highlighted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
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
    #[doc = "Change the `muted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
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
    #[doc = "Change the `openerTabId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
    pub fn opener_tab_id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("openerTabId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pinned` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
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
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`*"]
    pub fn url(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("url"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TabUpdateDetails {
    fn default() -> Self {
        Self::new()
    }
}
