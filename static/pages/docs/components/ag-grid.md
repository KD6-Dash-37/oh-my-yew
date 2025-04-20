---
title: "AG Grid"
date: "2025-04-15"
tags: ["aggrid", "bindings", "js"]
---

## 🧱 AG Grid Web Component Integration (Enterprise)

This was by far the most involved piece of the project — wiring up AG Grid (Enterprise) into a Yew/WASM app.

> 🧩 tl;dr — It works, but you'll pay a **complexity tax** to make it clean.

---

### 🛠️ Summary of Techniques

| Technique           | Why Used                                                                 |
|---------------------|--------------------------------------------------------------------------|
| JS Web Component    | AG Grid is too large for `wasm-bindgen` — wrap it as `<ag-grid-table>`  |
| UMD Build           | Simplifies loading without import/module headaches                      |
| `copy-dir` / `copy-file` | Avoids hashed filenames; lets us pick exactly what gets bundled   |
| `fetch-ag-grid.sh`  | Downloads modules from npm at build-time — nothing checked into Git     |

---

### 📁 Directory Layout

```
static/
├── js/
│   └── aggrid-wrapper.js       <-- Custom Web Component wrapper
└── vendor/
    ├── ag-grid-community/      <-- Entire module from npm
    └── ag-grid-enterprise/     <-- Entire module from npm
```

---

### ✅ Pros

- Clean separation: Yew drives the data, AG Grid handles the UI.
- Minimal config in `index.html`:

  ```html
  <link rel="copy-dir" href="static/vendor/ag-grid-community" />
  <link rel="copy-dir" href="static/vendor/ag-grid-enterprise" />
  <script src="/vendor/ag-grid-community/dist/ag-grid-community.min.js"></script>
  <script src="/vendor/ag-grid-enterprise/dist/ag-grid-enterprise.min.js"></script>
  <script type="module" src="static/js/aggrid-wrapper.js"></script>
  ```

- Reusable, idiomatic Rust Web Component:

  ```rust
  html! { <ag-grid-table data={...} columns={...} /> }
  ```

---

### ⚠️ Gotchas / Drawbacks

| Issue                         | Notes                                                                 |
|------------------------------|-----------------------------------------------------------------------|
| Manual asset management       | `copy-dir` and `data-target-path` must be used precisely              |
| Bundle size                  | Mitigated by scripting the fetch + cherry-picking just UMD/CSS assets |
| ESM import pain              | UMD is easier unless you're fully bundling with Webpack/Vite          |
| Enterprise mode dependency   | Requires UMD `<script>` to mutate global `agGrid`                     |
| JS version churn             | Theming API changes (v33) broke styling — required config updates      |

---

### 🔚 Final Thoughts

This approach works well *if* you treat AG Grid as a **black box** and let JS handle what it's good at.

`npm` is used only during fetch — not for compiling or bundling.

> 🦀 Let Rust handle your logic — let JS do what it does best in the UI layer.

---

## ✅ Checklist: Adding a New JS Web Component

- ✅ `1️⃣ Add Your JS Web Component`
- ✅ `2️⃣ Register in index.html`
- ✅ `3️⃣ Include External Dependencies`
- ✅ `4️⃣ Use in Yew Like Native HTML`
- ✅ `5️⃣ Trunk.toml (if missing)`
- ✅ `6️⃣ Final Build Validation`

---

### 1️⃣ Drop the JS file into `static/js/`

```js
// static/js/aggrid-wrapper.js
class AgGridTable extends HTMLElement {
  connectedCallback() {
    // setup AG Grid instance here
  }
}
customElements.define("ag-grid-table", AgGridTable);
```

---

### 2️⃣ Register it in `index.html`

```html
<script 
  data-trunk 
  src="static/js/my-new-component.js" 
  type="module" 
  data-target-path="js">
</script>
```

📝 Use `data-target-path="js"` to flatten it into `dist/js`.

---

### 3️⃣ If it has CSS/JS dependencies

```html
<link data-trunk
    rel="copy-file"
    href="static/vendor/lib/lib.css"
    data-target-path="vendor/lib/" />

<script data-trunk
    src="static/vendor/lib/lib.umd.min.js"
    data-target-path="vendor/lib/">
</script>
```

🔎 Use [`copy-file`](https://trunkrs.dev/assets/#copy-file) to avoid dragging in bloat from `node_modules`.

---

### 4️⃣ Use it like any HTML tag in Yew

```rust
html! {
    <ag-grid-table data={...} columns={...} />
    // data & columns are passed as JS-serializable props
}
```

---

### 5️⃣ Add a `Trunk.toml` (if missing)

```toml
[build]
target = "index.html"
```

---

### 6️⃣ Final build check

```bash
trunk clean
trunk build -v
trunk serve --open
```

🧪 Validate in browser:
- Correct paths?
- Web component registered?
- Network tab shows expected assets?

---

### 🧠 Summary Mental Model - New JS Web Component

| File/Dir            | Purpose                               | Notes |
|---------------------|---------------------------------------|-------|
| `static/js/*.js`    | Web Component source files            | Flat layout — 1 file per component |
| `index.html`        | Script & CSS registration via Trunk   | Explicit with `data-target-path` for clean dist structure |
| `static/vendor/`    | External JS/CSS dependencies          | Cherry-pick with `copy-file` or `copy-dir` — avoid npm bloat |
| Rust (Yew) code     | Uses Web Components as native HTML    | No bindings required unless calling JS functions |
| `Trunk.toml`        | Trunk build config                   | Define build target (`index.html`) |

