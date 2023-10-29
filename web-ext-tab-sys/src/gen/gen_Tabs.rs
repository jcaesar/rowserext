#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Tabs , typescript_type = "Tabs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Tabs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`*"]
    pub type Tabs;
    #[cfg(feature = "TabsCaptureVisibleTabDetails")]
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = captureVisibleTab)]
    #[doc = "The `captureVisibleTab()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/captureVisibleTab)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`, `TabsCaptureVisibleTabDetails`*"]
    pub fn capture_visible_tab(
        this: &Tabs,
        window_id: i32,
        details: &TabsCaptureVisibleTabDetails,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "TabCreateDetails")]
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = create)]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabCreateDetails`, `Tabs`*"]
    pub fn create(this: &Tabs, details: &TabCreateDetails) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`*"]
    pub fn get(this: &Tabs, tab_id: i32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = getCurrent)]
    #[doc = "The `getCurrent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/getCurrent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`*"]
    pub fn get_current(this: &Tabs) -> ::js_sys::Promise;
    #[cfg(feature = "TabQueryDetails")]
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = query)]
    #[doc = "The `query()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabQueryDetails`, `Tabs`*"]
    pub fn query(this: &Tabs, query_details: &TabQueryDetails) -> ::js_sys::Promise;
    #[cfg(feature = "TabReloadDetails")]
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = reload)]
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabReloadDetails`, `Tabs`*"]
    pub fn reload(this: &Tabs, details: &TabReloadDetails) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = remove)]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`*"]
    pub fn remove_with_i32(this: &Tabs, tab_ids: i32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = remove)]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`*"]
    pub fn remove_with_i32_sequence(
        this: &Tabs,
        tab_ids: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = sendMessage)]
    #[doc = "The `sendMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/sendMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Tabs`*"]
    pub fn send_message(
        this: &Tabs,
        tab_id: i32,
        message: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "TabUpdateDetails")]
    # [wasm_bindgen (method , structural , js_class = "Tabs" , js_name = update)]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Tabs/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TabUpdateDetails`, `Tabs`*"]
    pub fn update(this: &Tabs, tab_id: i32, details: &TabUpdateDetails) -> ::js_sys::Promise;
}
