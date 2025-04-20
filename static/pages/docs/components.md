---
title: "Components"
date: "2025-04-15"
tags: ["ag-grid", "web-components", "js"]
---


## 🆚 Rust WASM Bindings vs. JS Web Components

| Concept       | Rust WASM Bindings                                              | JS Web Components                                           |
|---------------|------------------------------------------------------------------|-------------------------------------------------------------|
| **Approach**  | Call JS code *directly* from Rust using `wasm-bindgen`          | Integrate existing JS UI libraries as custom HTML elements |
| **Example**   | Plotly Chart via `plotly.js` Rust bindings                      | AG Grid `<ag-grid-table>` custom element                   |
| **How it works** | Rust exposes JS functions via `extern "C"` block             | JS registers a Web Component class with its own lifecycle  |
| **Pros**      | Strong Rust type safety, no JS glue, idiomatic APIs            | Easy integration of large/complex JS libs with minimal Rust |
| **Cons**      | Requires bindings/crate support or writing your own             | JS still controls rendering; Rust becomes a host            |

