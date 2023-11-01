use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect_throw("no global `window` exists");
    let document = window
        .document()
        .expect_throw("should have a document on window");
    let body = document.body().expect_throw("document should have a body");
    body.style()
        .set_property("border", "5px solid red")
        .expect_throw("should be able to modify body style");

    Ok(())
}
