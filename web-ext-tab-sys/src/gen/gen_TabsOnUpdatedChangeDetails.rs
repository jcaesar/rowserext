#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabsOnUpdatedChangeDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TabsOnUpdatedChangeDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
    pub type TabsOnUpdatedChangeDetails;
}
impl TabsOnUpdatedChangeDetails {
    #[doc = "Construct a new `TabsOnUpdatedChangeDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
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
    #[doc = "Change the `favIconUrl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
    pub fn fav_icon_url(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("favIconUrl"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "TabMutedDetails")]
    #[doc = "Change the `mutedDetails` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabMutedDetails`, `TabsOnUpdatedChangeDetails`*"]
    pub fn muted_details(&mut self, val: Option<&TabMutedDetails>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mutedDetails"),
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
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
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
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
    pub fn status(&mut self, val: Option<&str>) -> &mut Self {
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
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `TabsOnUpdatedChangeDetails`*"]
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
impl Default for TabsOnUpdatedChangeDetails {
    fn default() -> Self {
        Self::new()
    }
}
