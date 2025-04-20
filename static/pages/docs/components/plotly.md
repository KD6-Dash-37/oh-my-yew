---
title: "Plotly"
date: "2025-04-15"
tags: ["plotly", "bindings", "js"]
---

## 📊 Plotly Chart via WASM Bindgen 

> ✅ If you have **high-quality bindings**, always prefer this approach over a JS Web Component.

### ✨ Key Benefits

- **Minimal `index.html` setup**  
  Just load Plotly from CDN:

  ```html
  <script src="https://cdn.plot.ly/plotly-2.27.0.min.js"></script>
  ```

- **Zero JS bundling / copying required**  
  Plotly is bound via:

  ```rust
  #[wasm_bindgen(module = "/static/js/plotly.js")]
  extern "C" {
      #[wasm_bindgen(js_name = "renderChart")]
      pub fn render_chart(...);
  }
  ```

---

### 🧠 Design Tips

- Use `use_effect` to reactively update the chart when state (`selected`) changes.
- Separate data selection logic (`get_chart_data_for_instrument`) from rendering.
- Yew's `<select>` integrates beautifully with state and works well with Plotly.

---

### ➕ How to Add a New Chart Page

1. Add your chart logic in a new `apps/<my_chart>/mod.rs`.
2. Define a matching JS wrapper in `static/js/my_chart.js`.
3. Bind with `#[wasm_bindgen(module = "...")]`.
4. Route it in `Route::MyChartApp` and add it to the sidebar.

---