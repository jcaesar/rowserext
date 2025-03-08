mod tabs;

use crate::tabs::TabQueryDetails;
use cached::proc_macro::cached;
use dioxus::prelude::*;
use futures::StreamExt;
#[allow(unused_imports)]
use gloo::console::log;
use gloo::timers::future::sleep;
use itertools::Itertools;
use js_sys::Array;
use once_cell::sync::Lazy;
use std::{borrow::Cow, cmp::Reverse, sync::Arc, time::Duration};
use tabs::{EagerTab, Tab, TabId};
use tldextract::TldExtractor;
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    LaunchBuilder::web().launch(|| {
        let mut get_tabs = use_resource(|| get_tabs());
        let mut display_max = use_signal(|| 20);
        let mut group_by = use_signal(|| GroupBy::Url);
        let out = match get_tabs() {
            Some(tabs) => {
                rsx! {
                    h1 { "Lionel" }
                    p {
                        "Clean up groups of tabs "
                        span {
                            style: "color: red;",
                            "WITHOUT UNDO"
                        }
                        ". Remember, all of this is trash anyway."
                    }
                    p {
                        "Group by:"
                        for &g in GroupBy::all() {
                            br {}
                            input {
                                checked: g == GroupBy::all()[0],
                                r#type: "radio",
                                name: "group_by_select",
                                id: "{g.name()}",
                                oninput: move |_| group_by.set(g),
                            }
                            label {
                                r#for: "{g.name()}",
                                "{g.name()}"
                            }
                        }
                    }
                    p {
                        "Display "
                        input {
                            oninput: move |event| {
                                if let Ok(n) = event.value().trim().parse() {
                                    display_max.set(n)
                                }
                            },
                            value: "20",
                        }
                        " tab groups."
                    }
                    p {
                        button {
                            onclick: move |_| get_tabs.restart(),
                            "Reload"
                        }
                    }
                    tab_group_output {
                        tabs: tabs,
                        group_by: group_by(),
                        display_max: display_max(),
                    }
                }
            }
            None => rsx! { p { "Retrieving tab list..." } },
        };
        out
    });
}

async fn get_tabs() -> Arc<Vec<EagerTab>> {
    let tabs = tabs::api().query(TabQueryDetails::new().pinned(false));
    let tabs = JsFuture::from(tabs)
        .await
        .expect("Get tab list failed")
        .dyn_into::<Array>()
        .expect("Tab query didn't return list");
    let tabs = tabs
        .iter()
        .map(|tab| tab.unchecked_into::<Tab>().eager())
        .sorted()
        .collect::<Vec<_>>()
        .into();
    tabs
}

#[component]
fn tab_group_output(tabs: Arc<Vec<EagerTab>>, group_by: GroupBy, display_max: usize) -> Element {
    let closed = use_signal(|| 0);
    let to_close = use_signal(|| 0);
    let close_tabs = use_coroutine(move |mut rx: UnboundedReceiver<Vec<TabId>>| {
        let mut to_close = to_close.clone();
        let mut closed = closed.clone();
        async move {
            let tabs = tabs::api();
            loop {
                while let Some(remove) = rx.next().await {
                    to_close.set(to_close() + remove.len());
                    for remove in remove {
                        JsFuture::from(tabs.remove(remove)).await.expect("TODO");
                        closed += 1;
                        sleep(Duration::from_millis(1)).await;
                    }
                }
            }
        }
    });
    let todo = format!(
        "{closed} closed, {} pending, {} total.",
        to_close() - closed(),
        tabs.len() - closed()
    );
    let keyname = group_by.name();
    rsx! {
        h2 { "Tabs by {keyname}" }
        p {
            "{todo}"
        }
        table {
            tr {
                id: "head",
                th { "Close" }
                th { "Count" }
                th {
                    style: "text-align: left;",
                    "{keyname}"
                }
            }
            tab_group_rows {
                tabs,
                group_by: group_by.clone(),
                display_max,
                close_tabs,
            }
        }
    }
}

#[component]
fn tab_group_rows(
    tabs: Arc<Vec<EagerTab>>,
    group_by: GroupBy,
    display_max: usize,
    close_tabs: Coroutine<Vec<u32>>,
) -> Element {
    log!("Re!");
    let same = group_tabs(tabs.to_vec(), group_by, display_max);
    rsx! {
        for (url, tabs) in same {
            Fragment {
                key: "{url}",
                tab_group {
                    id: url.to_string(),
                    url: url.to_string(),
                    tabs: tabs.to_vec(),
                    close_tabs,
                }
            }
        }
    }
}

#[cached(size = 1)]
fn group_tabs(
    tabs: Vec<EagerTab>,
    group_by: GroupBy,
    display_max: usize,
) -> Vec<(String, Vec<u32>)> {
    let mut same = tabs
        .iter()
        .into_grouping_map_by(|t| group_by.group(t))
        .fold(vec![], |mut acc, _key, tab| {
            acc.push(tab.id);
            acc
        });
    for (_, ids) in &mut same {
        ids.sort();
    }
    let same_count = same
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .into_group_map_by(|(_, v)| v.len())
        .into_iter()
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let mut same = vec![];
    for (_, groups) in same_count {
        if same.len() + groups.len() > display_max {
            break;
        }
        same.extend_from_slice(&groups);
    }
    same.sort_by_key(|(_key, tabs)| (Reverse(tabs.len()), tabs[0]));
    same
}

#[component]
fn tab_group(
    id: String,
    url: String,
    tabs: Vec<TabId>,
    close_tabs: Coroutine<Vec<u32>>,
) -> Element {
    let _ = id;
    let mut pressed = use_signal(|| false);
    let pressed_hidden = match pressed() {
        true => "visibility: hidden",
        false => "",
    };
    let pressed_crossedout = match pressed() {
        true => "text-decoration: line-through; text-color: #7f000;",
        false => "",
    };
    rsx! {
        tr {
            id: "{url}",
            td {
                style: pressed_hidden,
                button {
                    id: "{url}-all",
                    onclick: {
                        let tabs = tabs.clone();
                        move |_| {
                            if !pressed() {
                                close_tabs.send(tabs.clone());
                            }
                            pressed.set(true);
                        }
                    },
                    "all"
                }
                button {
                    id: "{url}-dups",
                    onclick: {
                        let tabs = tabs.clone();
                        move |_| {
                            if !pressed() {
                                close_tabs.send(tabs.iter().cloned().skip(1).collect());
                            }
                            pressed.set(true);
                        }
                    },
                    "dups"
                }
            }
            td {
                id: "{url}-count",
                style: "text-align: right; {pressed_crossedout}",
                "{tabs.len()}"
            }
            td { "{url}" }
        }
    }
}

static TLDEX: Lazy<TldExtractor> =
    Lazy::new(|| TldExtractor::new(tldextract::TldOption::default()));

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum GroupBy {
    Url,
    Origin,
    Domain,
}

impl GroupBy {
    fn all() -> &'static [Self] {
        &[GroupBy::Url, GroupBy::Origin, GroupBy::Domain]
    }
    fn name(&self) -> &str {
        match self {
            GroupBy::Url => "URL",
            GroupBy::Domain => "Domain",
            GroupBy::Origin => "Origin",
        }
    }
    fn group<'a>(&self, tab: &'a EagerTab) -> Cow<'a, str> {
        let url = Cow::Borrowed(tab.url.as_str());
        match self {
            GroupBy::Url => url,
            GroupBy::Origin => match Url::parse(&url) {
                Ok(parsed) => match parsed.origin() {
                    url::Origin::Opaque(_) => url,
                    url::Origin::Tuple(scheme, host, port) => match parsed.port().is_none() {
                        true => format!("{scheme}://{host}").into(),
                        false => format!("{scheme}://{host}:{port}").into(),
                    },
                },
                Err(_) => url,
            },
            GroupBy::Domain => match TLDEX.extract(&url) {
                Ok(tldextract::TldResult {
                    domain: Some(domain),
                    suffix: Some(suffix),
                    ..
                }) => format!("{domain}.{suffix}").into(),
                _ => GroupBy::Origin.group(tab),
            },
        }
    }
}
