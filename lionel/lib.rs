mod tabs;

use crate::tabs::{api, TabQueryDetails};
use dioxus::prelude::*;
use itertools::Itertools;
use js_sys::Array;
use tabs::Tab;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// TODO: open settings through browser.runtime.openOptionsPage() through https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/browserAction/onClicked

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    dioxus_web::launch(|cx| {
        let tabs = use_future(cx, (), |_| async move {
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
            tabs
        });
        let out = match tabs.value() {
            Some(tabs) => {
                rsx! {
                    tab_group_list {
                        tabs: tabs,
                        keyname: "URL",
                        grouper: |tab: &Tab| tab.url()
                    }
                }
            }
            None => rsx! { p { "Retrieving tab list..." } },
        };
        cx.render(out)
    });
}

#[inline_props]
fn tab_group_list<'a, F: Fn(&Tab) -> String>(
    cx: Scope<'a>,
    tabs: &'a [Tab],
    keyname: &'a str,
    grouper: F,
) -> Element<'a> {
    let same = tabs.into_iter().into_group_map_by(|t| grouper(t));
    let same = same
        .into_iter()
        .filter(|(_, tabs)| tabs.len() > 1)
        .sorted_by_key(|(_url, tabs)| tabs.len())
        .rev()
        .collect::<Vec<_>>();
    cx.render(rsx! {
        h2 { "Tabs by {keyname}" }
        table {
            tr {
                id: "head",
                th { "Count" }
                th { "{keyname}" }
            }
            for (url, tabs) in &same {
                tr {
                    id: "{url}",
                    td { "{tabs.len()}" }
                    td { "{url}" }
                }
            }
        }
    })
}
