#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WindowTypes` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WindowTypes`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowTypes {
    App = "app",
    Normal = "normal",
    Panel = "panel",
    Popup = "popup",
}
