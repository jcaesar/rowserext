mod event_stream;

use chrono::{DateTime, Duration, Local, NaiveTime};
use event_stream::EventStream;
use futures::{select, StreamExt};
use gloo::{
    dialogs::{alert, prompt},
    timers::future::IntervalStream,
    utils::{document, window},
};
use itertools::Itertools;
use std::iter;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlButtonElement, HtmlCollection};

fn collection_list(c: HtmlCollection) -> impl Iterator<Item = Element> {
    (0..).map_while(move |i| c.item(i))
}

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    let hash_changes = &mut EventStream::new(&window(), "hashchange");
    loop {
        let mut check = IntervalStream::new(300).take(300);
        while let Some(()) = check.next().await {
            if let Some(join_btn) = find_join_btn() {
                in_join_menu(hash_changes, &join_btn).await;
                break;
            }
        }
        hash_changes.next().await.unwrap_throw();
    }
}

async fn in_join_menu(hash_changes: &mut EventStream, join_btn: &HtmlButtonElement) {
    let our_btn = mk_our_btn(join_btn);
    let clicks = &mut EventStream::new(&our_btn, "click");
    let ivl = &mut IntervalStream::new(1_000).fuse();
    let mut join_by: Option<DateTime<Local>> = None;
    loop {
        select! {
            e = clicks.next() => {
                e.unwrap_throw();
                join_by = query_time();
            },
            e = hash_changes.next() => e.unwrap_throw(),
            e = ivl.next() => e.unwrap_throw(),
        }
        if find_join_btn().is_none() {
            break;
        }
        let now = Local::now();
        update_btn_text(join_by, now, &our_btn);
        if let Some(join_by) = join_by {
            if now >= join_by {
                join_btn.click();
                break;
            }
        }
    }
    if let Some(parent) = our_btn.parent_element() {
        parent.remove_child(&our_btn).ok();
    }
}

fn query_time() -> Option<DateTime<Local>> {
    let now = Local::now();
    let str = prompt(
        &format!(
            "Enter join time (HH:MM:SS) (Now: {})",
            now.time().format("%H:%M:%S")
        ),
        None,
    )?;
    let parse_from_str = chrono::NaiveTime::parse_from_str;
    let time = parse_from_str(&str, "%H:%M:%S").or(parse_from_str(&str, "%H:%M"));
    let Ok(time) = time else {
        alert(&format!("Couldn't parse {}", str));
        return None;
    };
    let rel = time - now.time();
    let rel = match rel >= Duration::seconds(0) {
        true => rel,
        false => rel + Duration::days(1),
    };
    Some(now + rel)
}

fn mk_our_btn(join_btn: &HtmlButtonElement) -> HtmlButtonElement {
    let our_btn = document()
        .create_element("button")
        .expect_throw("create element failed")
        .dyn_into::<HtmlButtonElement>()
        .expect_throw("button isn't a button");
    our_btn.set_type("button");
    our_btn.set_text_content(Some("…"));
    let add_classes = join_btn.class_list();
    for class in (0..).map_while(|i| add_classes.item(i)) {
        our_btn
            .class_list()
            .add_1(&class)
            .expect_throw("Can't add class");
    }
    our_btn.style().set_property("margin-left", "5px").ok();
    join_btn
        .parent_node()
        .expect_throw("Join button free floating")
        .append_child(&our_btn)
        .expect_throw("Can't append our button");
    our_btn
}

fn update_btn_text(
    join_by: Option<DateTime<Local>>,
    now: DateTime<Local>,
    our_btn: &HtmlButtonElement,
) {
    let var_text;
    let text = match join_by {
        Some(join_by) => {
            if now >= join_by {
                "Joining!"
            } else {
                let diff = NaiveTime::from_hms_opt(0, 0, 0).unwrap() + (join_by - now);
                var_text = format!(
                    "Joining at {} (in {}…)",
                    join_by.format("%H:%M:%S"),
                    diff.format("%H:%M:%S")
                );
                &var_text
            }
        }
        None => "Join by …",
    };
    our_btn.set_text_content(Some(text));
}

fn find_join_btn() -> Option<HtmlButtonElement> {
    iter::empty()
        .chain(
            document()
                .get_element_by_id("prejoin-join-button")
                .into_iter(),
        )
        .chain(collection_list(
            document().get_elements_by_class_name("join-btn"),
        ))
        .filter_map(|e| e.dyn_into::<HtmlButtonElement>().ok())
        .filter(|e| e.text_content().as_deref() == Some("Join now"))
        .exactly_one()
        .ok()
}
