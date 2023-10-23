(async () => {
    await load();
})();

async function load() {
	await wasm_bindgen(chrome.runtime.getURL('pkg/ext_bg.wasm'));
}
