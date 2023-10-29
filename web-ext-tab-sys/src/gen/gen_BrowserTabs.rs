#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = BrowserTabs , typescript_type = "BrowserTabs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserTabs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`*"]
    pub type BrowserTabs;
    #[cfg(feature = "TabsCaptureVisibleTabDetails")]
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = captureVisibleTab)]
    #[doc = "The `captureVisibleTab()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/captureVisibleTab)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`, `TabsCaptureVisibleTabDetails`*"]
    pub fn capture_visible_tab(
        this: &BrowserTabs,
        window_id: i32,
        details: &TabsCaptureVisibleTabDetails,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "TabCreateDetails")]
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = create)]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`, `TabCreateDetails`*"]
    pub fn create(this: &BrowserTabs, details: &TabCreateDetails) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`*"]
    pub fn get(this: &BrowserTabs, tab_id: i32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = getCurrent)]
    #[doc = "The `getCurrent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/getCurrent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`*"]
    pub fn get_current(this: &BrowserTabs) -> ::js_sys::Promise;
    #[cfg(feature = "TabQueryDetails")]
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = query)]
    #[doc = "The `query()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`, `TabQueryDetails`*"]
    pub fn query(this: &BrowserTabs, query_details: &TabQueryDetails) -> ::js_sys::Promise;
    #[cfg(feature = "TabReloadDetails")]
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = reload)]
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`, `TabReloadDetails`*"]
    pub fn reload(this: &BrowserTabs, details: &TabReloadDetails) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = remove)]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`*"]
    pub fn remove_with_i32(this: &BrowserTabs, tab_ids: i32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = remove)]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`*"]
    pub fn remove_with_i32_sequence(
        this: &BrowserTabs,
        tab_ids: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = sendMessage)]
    #[doc = "The `sendMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/sendMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`*"]
    pub fn send_message(
        this: &BrowserTabs,
        tab_id: i32,
        message: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "TabUpdateDetails")]
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = update)]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserTabs/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserTabs`, `TabUpdateDetails`*"]
    pub fn update(this: &BrowserTabs, tab_id: i32, details: &TabUpdateDetails)
        -> ::js_sys::Promise;
}
