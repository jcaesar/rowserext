use js_sys::Array;
use wasm_bindgen::prelude::*;

pub fn api() -> Tabs {
    Err(())
        .or_else(|_| Tabs::browser_get_tabs())
        .or_else(|_| Tabs::chrome_get_tabs())
        .expect("Not in a tabs supporting browser's web extension")
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Tabs )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Tabs;
    #[wasm_bindgen(getter, catch, static_method_of = Tabs, js_class = browser, js_name = tabs)]
    pub fn browser_get_tabs() -> Result<Tabs, JsValue>;
    #[wasm_bindgen(getter, catch, static_method_of = Tabs, js_class = chrome, js_name = tabs)]
    pub fn chrome_get_tabs() -> Result<Tabs, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = query)]
    pub fn query(this: &Tabs, query_details: &TabQueryDetails) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = remove)]
    pub fn remove(this: &Tabs, id: u32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = remove)]
    pub fn remove_with_array(this: &Tabs, ids: Array) -> ::js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabQueryDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TabQueryDetails;
}

macro_rules! setter {
    ($name:ident) => {
        pub fn $name(&mut self, val: bool) -> &mut Self {
            use wasm_bindgen::JsValue;
            let r = ::js_sys::Reflect::set(
                self.as_ref(),
                &JsValue::from(stringify!($name)),
                &JsValue::from(val),
            );
            debug_assert!(
                r.is_ok(),
                "setting properties should never fail on our dictionary objects"
            );
            let _ = r;
            self
        }
    };
}

impl TabQueryDetails {
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    setter!(pinned);
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Tab)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Tab;
    # [wasm_bindgen (structural , method , getter , js_class = "Tab" , js_name = url)]
    pub fn title(this: &Tab) -> String;
    # [wasm_bindgen (structural , method , getter , js_class = "Tab" , js_name = url)]
    pub fn url(this: &Tab) -> String;
    # [wasm_bindgen (structural , method , getter , js_class = "Tab" , js_name = id)]
    pub fn id(this: &Tab) -> u32;
}

impl Tab {
    pub fn eager(&self) -> EagerTab {
        EagerTab {
            title: self.title(),
            url: self.url(),
            id: self.id(),
        }
    }
}

pub type TabId = u32;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct EagerTab {
    pub id: TabId,
    pub url: String,
    pub title: String,
}
