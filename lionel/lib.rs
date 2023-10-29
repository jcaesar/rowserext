#![allow(unused)]

use chrono::{DateTime, Duration, Local, NaiveTime};
use futures::{select, StreamExt};
use gloo::{
    console::log,
    dialogs::{alert, prompt},
    timers::future::IntervalStream,
    utils::{document, window},
};
use itertools::Itertools;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_ext_tab_sys;
use web_sys::{Element, HtmlButtonElement, HtmlCollection};

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    document();
    let tabs = tabs().query(TabQueryDetails::new().pinned(false));
    let tabs = JsFuture::from(tabs).await.expect("Get tab list failed");
    log!(tabs);
}

pub fn tabs() -> Tabs {
    Tabs::get_tabs().expect("Not in a tabs supporting browser's web extension")
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Tabs )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Tabs;
    #[wasm_bindgen(getter, catch, static_method_of = Tabs, js_class = browser, js_name = tabs)]
    pub fn get_tabs() -> Result<Tabs, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "BrowserTabs" , js_name = query)]
    pub fn query(this: &Tabs, query_details: &TabQueryDetails) -> ::js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TabQueryDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TabQueryDetails;
}
impl TabQueryDetails {
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
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
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Tab)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Tab;
}
impl Tab {
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    pub fn active(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("active"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
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
    pub fn height(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("height"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    pub fn highlighted(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("highlighted"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    pub fn id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("incognito"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    pub fn index(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("index"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    pub fn opener_tab_id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("openerTabId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
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
    pub fn session_id(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sessionId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
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
    pub fn width(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("width"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
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
impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
