mod tabs;

use crate::tabs::{api, TabQueryDetails};
use gloo::console::log;
use itertools::Itertools;
use js_sys::Array;
use tabs::Tab;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    let tabs = api().query(TabQueryDetails::new().pinned(false));
    let tabs = JsFuture::from(tabs)
        .await
        .expect("Get tab list failed")
        .dyn_into::<Array>()
        .expect("Tab query didn't return list");
    let tabs = tabs
        .iter()
        .map(|tab| tab.unchecked_into())
        .collect::<Vec<Tab>>();
    let same_url = tabs.iter().into_group_map_by(|tab| tab.url());
    for (url, tabs) in same_url {
        log!(url, tabs.len());
    }
}
