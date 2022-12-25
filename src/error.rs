#[derive(Debug)]
pub enum AppError {
    IO(std::io::Error),
    BadPublicKey,
    WasmError(wasm_bindgen::JsValue),
    WasmError2(wasm_bindgen::JsValue),
    WasmError3(wasm_bindgen::JsValue),
    SerdeJsonError(serde_json::Error),
}
