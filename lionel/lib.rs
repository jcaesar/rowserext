mod tabs;

use crate::tabs::{api, TabQueryDetails};
use gloo::{console::log, utils::document};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    document();
    let tabs = api().query(TabQueryDetails::new().pinned(false));
    let tabs = JsFuture::from(tabs).await.expect("Get tab list failed");
    log!(tabs);
}
