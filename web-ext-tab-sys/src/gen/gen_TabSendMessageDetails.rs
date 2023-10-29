#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabSendMessageDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabSendMessageDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabSendMessageDetails`*"]
    pub type TabSendMessageDetails;
}
impl TabSendMessageDetails {
    #[doc = "Construct a new `TabSendMessageDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabSendMessageDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `frameId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabSendMessageDetails`*"]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("frameId"),
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
impl Default for TabSendMessageDetails {
    fn default() -> Self {
        Self::new()
    }
}
