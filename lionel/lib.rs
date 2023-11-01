mod tabs;

use crate::tabs::TabQueryDetails;
use dioxus::prelude::*;
use futures::StreamExt;
use gloo::timers::future::sleep;
use itertools::Itertools;
use js_sys::Array;
use once_cell::sync::Lazy;
use std::{cmp::Reverse, time::Duration};
use tabs::Tab;
use tldextract::TldExtractor;
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    dioxus_web::launch(|cx| {
        let get_tabs = use_future(cx, (), |()| get_tabs());
        let display_max = use_state(cx, || 20);
        let group_by = use_state(cx, || GroupBy::Url);
        let out = match get_tabs.value() {
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
                            oninput: |event| {
                                if let Ok(n) = event.value.trim().parse() {
                                    display_max.set(n)
                                }
                            },
                            value: "20",
                        }
                        " tab groups."
                    }
                    p {
                        button {
                            onclick: |_| get_tabs.restart(),
                            "Reload"
                        }
                    }
                    tab_group_list {
                        tabs: tabs,
                        group_by: **group_by,
                        display_max: **display_max,
                    }
                }
            }
            None => rsx! { p { "Retrieving tab list..." } },
        };
        cx.render(out)
    });
}

async fn get_tabs() -> Vec<Tab> {
    let tabs = tabs::api().query(TabQueryDetails::new().pinned(false));
    let tabs = JsFuture::from(tabs)
        .await
        .expect("Get tab list failed")
        .dyn_into::<Array>()
        .expect("Tab query didn't return list");
    let tabs = tabs.iter().map(|tab| tab.unchecked_into()).collect();
    tabs
}

type TabId = u32;

#[inline_props]
fn tab_group_list<'a>(
    cx: Scope<'a>,
    tabs: &'a [Tab],
    group_by: GroupBy,
    display_max: usize,
) -> Element<'a> {
    let mut same = tabs
        .iter()
        .into_grouping_map_by(|t| group_by.group(t))
        .fold(vec![], |mut acc, _key, tab| {
            acc.push(tab.id());
            acc
        });
    for (_, ids) in &mut same {
        ids.sort();
    }
    let same_count = same
        .iter()
        .into_group_map_by(|(_, v)| v.len())
        .into_iter()
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let mut same = vec![];
    for (_, groups) in same_count {
        if same.len() + groups.len() > *display_max {
            break;
        }
        same.extend_from_slice(&groups);
    }
    same.sort_by_key(|(key, tabs)| (Reverse(tabs.len()), *key, tabs[0]));
    let closed = use_state(cx, || 0);
    let to_close = use_state(cx, || 0);
    let close_tabs = use_coroutine(cx, |mut rx: UnboundedReceiver<Vec<TabId>>| {
        let mut closed = closed.clone();
        async move {
            loop {
                while let Some(remove) = rx.next().await {
                    for remove in remove {
                        JsFuture::from(tabs::api().remove(remove))
                            .await
                            .expect("TODO");
                        closed += 1;
                        sleep(Duration::from_millis(1)).await;
                    }
                }
            }
        }
    });
    let show_count = same.iter().map(|(_, v)| v.len()).sum::<usize>();
    let todo = match **to_close {
        0 => String::new(),
        _ => format!(" Closed {closed}, {} pending.", **to_close - **closed),
    };
    let keyname = group_by.name();
    cx.render(rsx! {
        h2 { "Tabs by {keyname}" }
        p {
            "Showing {show_count} of {tabs.len()}.{todo}"
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
            for (url, tabs) in same {
                tab_group {
                    url: url.to_string(),
                    tabs: tabs.to_vec(),
                    close_tabs: move |tabs| {
                        to_close.set(*to_close.current() + tabs.len()); // += causes lifetime issues. -.-
                        close_tabs.send(tabs);
                    },
                }
            }
        }
    })
}

#[inline_props]
fn tab_group<F: Fn(Vec<TabId>)>(
    cx: Scope<'a>,
    close_tabs: F,
    url: String,
    tabs: Vec<TabId>,
) -> Element {
    let pressed = use_state(cx, || false);
    let pressed_hidden = match **pressed {
        true => "visibility: hidden",
        false => "",
    };
    let pressed_crossedout = match **pressed {
        true => "text-decoration: line-through; text-color: #7f000;",
        false => "",
    };
    cx.render(rsx! {
        tr {
            id: "{url}",
            td {
                style: pressed_hidden,
                button {
                    id: "{url}-all",
                    onclick: move |_| {
                        pressed.set(true);
                        if !pressed {
                            close_tabs(tabs.clone());
                        }
                    },
                    "all"
                }
                button {
                    id: "{url}-dups",
                    onclick: move |_| {
                        pressed.set(true);
                        if !pressed {
                            close_tabs(tabs.iter().cloned().skip(1).collect());
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
    })
}

static TLDEX: Lazy<TldExtractor> =
    Lazy::new(|| TldExtractor::new(tldextract::TldOption::default()));

#[derive(Copy, Clone, PartialEq, Eq)]
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
    fn group(&self, tab: &Tab) -> String {
        let url = tab.url();
        match self {
            GroupBy::Url => url,
            GroupBy::Origin => match Url::parse(&url) {
                Ok(parsed) => match parsed.origin() {
                    url::Origin::Opaque(_) => url,
                    url::Origin::Tuple(scheme, host, port) => match parsed.port().is_none() {
                        true => format!("{scheme}://{host}"),
                        false => format!("{scheme}://{host}:{port}"),
                    },
                },
                Err(_) => url,
            },
            GroupBy::Domain => match TLDEX.extract(&url) {
                Ok(tldextract::TldResult {
                    domain: Some(domain),
                    suffix: Some(suffix),
                    ..
                }) => format!("{domain}.{suffix}"),
                _ => GroupBy::Origin.group(tab),
            },
        }
    }
}
