#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabIdWindowId)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabIdWindowId` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabIdWindowId`*"]
    pub type TabIdWindowId;
}
impl TabIdWindowId {
    #[doc = "Construct a new `TabIdWindowId`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabIdWindowId`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `tabId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabIdWindowId`*"]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("tabId"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `windowId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabIdWindowId`*"]
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
impl Default for TabIdWindowId {
    fn default() -> Self {
        Self::new()
    }
}