#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabScriptAndCSSDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabScriptAndCssDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
    pub type TabScriptAndCssDetails;
}
impl TabScriptAndCssDetails {
    #[doc = "Construct a new `TabScriptAndCssDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `allFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("allFrames"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
    pub fn code(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("code"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `file` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
    pub fn file(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("file"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `frameId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
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
    #[doc = "Change the `matchAboutBlank` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabScriptAndCssDetails`*"]
    pub fn match_about_blank(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("matchAboutBlank"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RunAt")]
    #[doc = "Change the `runAt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RunAt`, `TabScriptAndCssDetails`*"]
    pub fn run_at(&mut self, val: Option<RunAt>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("runAt"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TabScriptAndCssDetails {
    fn default() -> Self {
        Self::new()
    }
}
