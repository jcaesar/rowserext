#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `TabStatus` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TabStatus`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStatus {
    Complete = "complete",
    Loading = "loading",
}
