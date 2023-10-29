#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabMutedDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabMutedDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabMutedDetails`*"]
    pub type TabMutedDetails;
}
impl TabMutedDetails {
    #[doc = "Construct a new `TabMutedDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabMutedDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `extensionId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabMutedDetails`*"]
    pub fn extension_id(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("extensionId"),
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
    #[doc = "*This API requires the following crate features to be activated: `TabMutedDetails`*"]
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
    #[cfg(feature = "TabMutedReasonDetails")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabMutedDetails`, `TabMutedReasonDetails`*"]
    pub fn reason(&mut self, val: Option<TabMutedReasonDetails>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("reason"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TabMutedDetails {
    fn default() -> Self {
        Self::new()
    }
}
