use thiserror::Error;
#[derive(Error, Debug)]
pub enum GraphvizmError {
    #[error("Wasmtime error: {0}")]
    Wasmtime(#[from] wasmtime::Error),
    #[error("Graphviz render failed: {0}")]
    Render(String),
    #[error("Failed to initialize Graphvizm engine: {0}")]
    EngineInit(String),
    #[error("Failed to read Wasm component: {0}")]
    WasmRead(#[from] std::io::Error),
}
