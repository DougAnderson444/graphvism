# Graphvizm

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> _Portable Graphviz anywhere WebAssembly (WASI) runs, in any language you prefer_

Graphvizm builds Graphviz as a WebAssembly Component (WASI Preview 2), enabling native and cross-platform usage without relying on emscripten or JavaScript. Inspired by [viz.js](https://github.com/mdaines/viz.js), but designed for modern WASI environments.

This allows you to use Graphviz's layout and rendering capabilities from any language that supports the Wasm component model, such as Rust, Python, Go, and more.

### Download 

Pre-built version is on the [Releases](https://github.com/DougAnderson444/graphvism/releases) page.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Build](#build)
- [Reference Implementation: Rust Crate](#reference-implementation-rust-crate)
- [Low-Level Usage Example](#low-level-usage-example)
- [API Overview](#api-overview)
- [Demo](#demo)
- [License](#license)
- [Maintainers](#maintainers)

## Features

*   Builds Graphviz as a self-contained, portable Wasm component (`viz.wasm`).
*   Exposes a clean, high-level API defined with WIT.
*   Create and manipulate graphs programmatically (add nodes, edges, set attributes).
*   Render graphs from DOT strings directly to SVG and other formats.
*   Supports multiple layout engines (`dot`, `neato`, etc.).
*   Runs in any WASI Preview 2 compatible runtime (e.g., Wasmtime).

## Prerequisites

Before building, you need to install the following tools:

*   **make**: For running the build process.
*   **curl**: For downloading dependencies.
*   **[wasi-sdk](https://github.com/WebAssembly/wasi-sdk)**: Required for compiling C/C++ code to WASI. The `Makefile` expects it to be at `/opt/wasi-sdk`. You can override this by setting the `WASI_SDK_PATH` environment variable.
*   **[wit-bindgen-cli](https://github.com/bytecodealliance/wit-bindgen)**: For generating bindings from the WIT definition. Install with `cargo install wit-bindgen-cli`.
*   **Rust and Cargo**: Required to run the demo and the reference implementation.

## Build

Clone the repository and run the main build command:

```sh
make
```

This command will:
1.  Download the source code for Graphviz and its dependency (Expat).
2.  Configure and build them for the `wasm32-wasi` target.
3.  Generate C bindings from the `wit/world.wit` file using `wit-bindgen`.
4.  Compile the C wrapper code (`src/component.c`) and link it against the Graphviz libraries.
5.  Adapt the resulting core Wasm module to a WASI Preview 2 component, creating `viz.wasm`.

The final component will be located at the root of the project: `viz.wasm`.

To clean up build artifacts, you can run `make clean`. To remove downloaded sources as well, run `make super-clean`.

## Reference Implementation: Rust Crate

For Rust developers, the easiest way to use this component is through the provided **`graphvizm`** crate, which serves as the reference implementation.

The crate bundles the pre-compiled `viz.wasm` component and offers a high-level, idiomatic Rust API that handles all the low-level `wasmtime` setup for you.

**➡️ For installation and usage instructions, see the crate's README: [crates/graphvizm/README.md](./crates/graphvizm/README.md)**

## Low-Level Usage Example

The build process produces a Wasm component `viz.wasm`. You can use this component in any environment that supports WASI Preview 2. For Rust developers, we recommend using the [reference implementation crate](#reference-implementation-rust-crate) for a simpler experience.

The following example demonstrates how to load and interact with the component directly using the `wasmtime` crate:

```rust
use anyhow::Context;
use wasmtime::{Config, Engine, Result, Store, component::{Component, Linker}};
use wasmtime_wasi::{self, ResourceTable, WasiCtx, WasiCtxView, WasiView};

// Generate bindings from the world.wit file
bindgen!("viz" in "wit/world.wit");

struct MyState {
    wasi: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.table }
    }
}

fn main() -> Result<()> {
    // Setup wasmtime engine and store
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker: Linker<MyState> = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    
    let wasi = WasiCtx::builder().inherit_stdio().build();
    let mut store = Store::new(&engine, MyState { wasi, table: ResourceTable::new() });

    // Load the component
    let component = Component::from_file(&engine, "viz.wasm")?;
    let viz = Viz::instantiate(&mut store, &component, &linker)?;
    let iface = &viz.interface0;

    // Define a simple graph in DOT format
    let simple_digraph = r#" digraph G { A -> B; } "#;

    // Create a context and render the graph
    let ctx = iface.call_create_context(&mut store)?;
    match iface.call_render_dot(&mut store, ctx, simple_digraph, "dot", "svg") {
        Ok(Ok(svg)) => {
            println!("Rendered SVG:\n{}", svg);
        }
        Ok(Err(err_str)) => eprintln!("Render failed: {}", err_str),
        Err(e) => eprintln!("Render error: {:?}", e),
    }

    Ok(())
}
```

For a more complete example, see the [Demo](#demo) section.

## API Overview

The component exports a `viz-api` interface defined in `wit/world.wit`. It provides functions to:

*   **Manage Graphs**: `create-graph`, `read-one-graph`, `add-node`, `add-edge`, `add-subgraph`.
*   **Set Attributes**: `set-default-graph-attribute`, `set-default-node-attribute`, `set-default-edge-attribute`, `set-attribute`.
*   **Render**: `layout`, `render`, and the convenience function `render-dot`.
*   **Inspect**: `get-graphviz-version`, `get-plugin-list`.

Resources like `graph`, `node`, `edge`, and `context` are managed as opaque handles.

## Demo

A complete Rust demo is provided in the `demo/` directory. It demonstrates creating a graph programmatically, setting attributes, and rendering it.

To run the demo:

```sh
# Make sure you have built the component first
make

# Run the demo
cd demo
cargo run
```

## License

MIT © Doug Anderson

## Maintainers

Doug Anderson
