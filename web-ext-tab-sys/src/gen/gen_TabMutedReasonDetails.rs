#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `TabMutedReasonDetails` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TabMutedReasonDetails`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabMutedReasonDetails {
    User = "user",
    Capture = "capture",
    Extension = "extension",
}
