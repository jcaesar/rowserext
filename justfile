default:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --target=no-modules --out-dir=pkg target/wasm32-unknown-unknown/debug/ext.wasm

release:
	cargo build --target wasm32-unknown-unknown --profile maxopt
	wasm-bindgen --target=no-modules --out-dir=pkg target/wasm32-unknown-unknown/maxopt/ext.wasm
