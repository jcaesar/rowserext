use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local};
use gloo::{
    dialogs::{alert, prompt},
    timers::callback::Interval,
    utils::document,
};
use itertools::Itertools;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlButtonElement, HtmlCollection};

fn collection_list(c: HtmlCollection) -> impl Iterator<Item = Element> {
    (0..).map_while(move |i| c.item(i))
}

#[wasm_bindgen(start)]
pub fn main() {
    mein().expect_throw("Failed")
}

fn mein() -> anyhow::Result<()> {
    our_btn()?;
    Ok(())
}

fn our_btn() -> Result<HtmlButtonElement> {
    const OUR_ID: &str = "ontimeebc54e68b53d30f5a82f0374edb391f6295b57586dff9a7b";
    match document().get_element_by_id(OUR_ID) {
        Some(btn) => Ok(btn
            .dyn_into()
            .expect_throw("Timed join button exists, but its weird")),
        None => {
            let join_btn = join_btn().context("Main join button not found")?;
            let our_btn = document()
                .create_element("button")
                .expect_throw("create element failed")
                .dyn_into::<HtmlButtonElement>()
                .expect_throw("button isn't a button");
            our_btn.set_id(OUR_ID);
            our_btn.set_type("button");
            our_btn.set_text_content(Some("…"));
            let add_classes = join_btn.class_list();
            for class in (0..).map_while(|i| add_classes.item(i)) {
                our_btn
                    .class_list()
                    .add_1(&class)
                    .expect_throw("Can't add class");
            }
            join_btn
                .parent_node()
                .expect_throw("Join button free floating")
                .append_child(&our_btn)
                .expect_throw("Can't append our button");
            let our_btn_outr = our_btn.clone();
            let join_by: Arc<Mutex<Option<DateTime<Local>>>> = Arc::new(Mutex::new(None));
            let join_by_cb = join_by.clone();
            let cb = move || {
                *join_by_cb.lock().unwrap() = None;
                let now = Local::now();
                let prompt = prompt(
                    &format!(
                        "Enter join time (HH:MM:SS) (Now: {})",
                        now.time().format("%H:%M:%S")
                    ),
                    None,
                );
                let Some(str) = prompt else {
                    return;
                };
                let parse_from_str = chrono::NaiveTime::parse_from_str;
                let time = parse_from_str(&str, "%H:%M:%S").or(parse_from_str(&str, "%H:%M"));
                let Ok(time) = time else {
                    alert(&format!("Couldn't parse {}", str));
                    return;
                };
                let rel = time - now.time();
                match rel >= Duration::seconds(0) {
                    true => rel,
                    false => rel + Duration::days(1),
                };
                *join_by_cb.lock().unwrap() = Some(now + rel);
            };
            let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn()>);
            our_btn_outr.set_onclick(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
            let our_btn_ivl = our_btn_outr.clone();
            Interval::new(1_000, move || {
                let var_text;
                let text = match *join_by.lock().unwrap() {
                    Some(join_by) => {
                        let now = Local::now();
                        if now >= join_by {
                            join_btn.click();
                            "Joining!"
                        } else {
                            let diff = (join_by - now).num_seconds();
                            var_text = format!(
                                "Joining in {:02}:{:02}:{:02}…",
                                diff / 3600,
                                diff / 60 % 60,
                                diff % 60
                            );
                            &var_text
                        }
                    }
                    None => "Join by …",
                };

                our_btn_ivl.set_text_content(Some(text));
            })
            .forget();
            Ok(our_btn)
        }
    }
}

fn join_btn() -> Option<HtmlButtonElement> {
    collection_list(document().get_elements_by_class_name("join-btn"))
        .filter_map(|e| e.dyn_into::<HtmlButtonElement>().ok())
        .filter(|e| e.text_content().as_deref() == Some("Join now"))
        .exactly_one()
        .ok()
}
