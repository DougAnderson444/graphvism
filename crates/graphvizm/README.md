# `graphvizm` 🦀

[![crates.io](https://img.shields.io/crates/v/graphvizm.svg)](https://crates.io/crates/graphvizm)
[![docs.rs](https://docs.rs/graphvizm/badge.svg)](https://docs.rs/graphvizm)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](../../LICENSE.md)

A simple and convenient Rust library for rendering [Graphviz](https://graphviz.org/) DOT language strings into SVG images.

This crate wraps a WebAssembly component that contains the Graphviz C library, providing safe and sandboxed access to Graphviz's powerful layout engines via the [Wasmtime](https://wasmtime.dev/) runtime.

## ✨ Features

- **Simple API:** A straightforward, easy-to-use API. Initialize with `Graphvizm::default()` and render with `render_dot()`.
- **Safe & Sandboxed:** By running Graphviz inside a Wasm sandbox, your application is protected from any potential issues in the underlying C library.
- **Self-Contained:** The compiled Graphviz Wasm component is bundled directly into the crate. Users on `crates.io` do not need to install the Graphviz C library or any C/C++ toolchain.
- **Efficient:** The Wasmtime engine and component are initialized once, allowing for efficient, repeated rendering of graphs.

## 🚀 Usage

1.  Add `graphvizm` to your `Cargo.toml`:

    ```toml
    [dependencies]
    graphvizm = "0.1.0" # Replace with the latest version
    ```

2.  Use it in your project to render a DOT string to SVG:

    ```rust
    use graphvizm::Graphvizm;
    use std::fs;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize the Graphvizm engine and component.
        let graphvizm = Graphvizm::new()?;

        let dot_string = r#"
            digraph G {
                rankdir="LR";
                node [shape=box, style=rounded];
                a [label="Client"];
                b [label="Rust 🦀"];
                c [label="Wasm 🕸️"];
                d [label="Graphviz 📊"];

                a -> b -> c -> d;
            }
        "#;

        // Render the DOT string to an SVG.
        let svg_output = graphvizm.render_dot(dot_string)?;

        // Save the output to a file.
        fs::write("graph.svg", svg_output)?;

        println!("✅ Graph saved to graph.svg");
        Ok(())
    }
    ```

    This will produce an SVG file named `graph.svg` in the current directory.

## 🛠️ For Developers (Building from Source)

This crate vendors a pre-compiled WebAssembly component (`viz.wasm`). If you have cloned the repository and wish to rebuild this component from the C sources, please see the instructions in the [root README.md](../../README.md).

After rebuilding the component, you must copy it into this crate's `vendor` directory to update it:

```sh
# From the repository root
cp viz.wasm crates/graphvizm/vendor/viz.wasm
```

## 📜 License

This project is licensed under the Apache-2.0 License.