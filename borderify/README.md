# Rust Web Extensions

This is a demo repository.

## Can you?

Yes, see the above reproducing [MDN's hello world extension](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_first_WebExtension) in Rust (plus a small shim for loading).

## How can you?

1. `$ just release`
2. Load the extension from [about:debugging#/runtime/this-firefox](about:debugging#/runtime/this-firefox)
3. Navigate to [mozilla.org](https://www.mozilla.org/) and feel the red carpet

## Should you?

No.

* Writing `web_sys` code is terribly verbose, and much more pain than joy. While a few things can be made nicer by `gloo`, e.g. `log!`, some things remain terrible, e.g. memory management for callbacks from JS to Rust.
* APIs you'd commonly use in a web extension (e.g. `browser`) probably don't have rust bindings yet. You may be able to [create those yourself](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/js_namespace.html), but that definitely isn't joy.

## Further trouble

* This is a very simple extension, with only a single content script. Normally, you'd have several different kinds of scripts. I'm not sure whether it's nicer to create a rust crate for each, or to use the same crate and differentiate from `load.js`.
* This is based on wasm-bindgen's `--target no-modules`, meaning you can't use js snippets.
