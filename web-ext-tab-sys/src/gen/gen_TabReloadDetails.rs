#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabReloadDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabReloadDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabReloadDetails`*"]
    pub type TabReloadDetails;
}
impl TabReloadDetails {
    #[doc = "Construct a new `TabReloadDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabReloadDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bypassCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabReloadDetails`*"]
    pub fn bypass_cache(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bypassCache"),
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
impl Default for TabReloadDetails {
    fn default() -> Self {
        Self::new()
    }
}
