#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabsCaptureVisibleTabDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabsCaptureVisibleTabDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsCaptureVisibleTabDetails`*"]
    pub type TabsCaptureVisibleTabDetails;
}
impl TabsCaptureVisibleTabDetails {
    #[doc = "Construct a new `TabsCaptureVisibleTabDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsCaptureVisibleTabDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "TabsCaptureVisibleTabFormat")]
    #[doc = "Change the `imageCaptureFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsCaptureVisibleTabDetails`, `TabsCaptureVisibleTabFormat`*"]
    pub fn image_capture_format(&mut self, val: TabsCaptureVisibleTabFormat) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("imageCaptureFormat"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `jpegQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsCaptureVisibleTabDetails`*"]
    pub fn jpeg_quality(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("jpegQuality"),
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
impl Default for TabsCaptureVisibleTabDetails {
    fn default() -> Self {
        Self::new()
    }
}
