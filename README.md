# 🚀 My Yew App

*A learn-as-you-go project to introduce myself to WASM using the Rust [Yew](https://yew.rs/) Framework — documenting different approaches for future reference.*

- ✅ Basic page navigation links  
- 🔧 Rust-implemented Yew components  
- 🔄 JS Interoperability
  - 🧪 WASM bindings to generate a Plotly chart  
  - 🧩 JS Web Component (AG Grid) integration
- 📚 Static Content System
  - Markdown files (`static/content/*.md`) with frontmatter metadata  
  - Rust `build.rs` generates a docs index at compile time  
  - Fully static site — no runtime fetches  
  - Hot reload supported for content edits via Trunk

If you want to run this repo locally, you'll need Rust and `npm`.  
👉 `npm` is only used at *fetch time* to download JS libraries — not at build or runtime.

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
# run the bash scripts to fetch AG-Grid (only necessary if you want to see the rendered grid)
trunk serve --open
```

---

## 🆚 Rust WASM Bindings vs. JS Web Components

| Concept       | Rust WASM Bindings                                              | JS Web Components                                           |
|---------------|------------------------------------------------------------------|-------------------------------------------------------------|
| **Approach**  | Call JS code *directly* from Rust using `wasm-bindgen`          | Integrate existing JS UI libraries as custom HTML elements |
| **Example**   | Plotly Chart via `plotly.js` Rust bindings                      | AG Grid `<ag-grid-table>` custom element                   |
| **How it works** | Rust exposes JS functions via `extern "C"` block             | JS registers a Web Component class with its own lifecycle  |
| **Pros**      | Strong Rust type safety, no JS glue, idiomatic APIs            | Easy integration of large/complex JS libs with minimal Rust |
| **Cons**      | Requires bindings/crate support or writing your own             | JS still controls rendering; Rust becomes a host            |

---

## 📊 Plotly Chart via WASM Bindgen (`chart.rs`)

> ✅ If you have **high-quality bindings**, always prefer this approach over a Web Component.

### ✨ Key Benefits

- **Minimal `index.html` setup**  
  Just load Plotly from CDN:

  ```html
  <script src="https://cdn.plot.ly/plotly-2.27.0.min.js"></script>
  ```

- **Zero JS bundling / copying required**  
  Plotly is bound via:

  ```rust
  #[wasm_bindgen(module = "/static/js/chart.js")]
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

> Let’s say you want to add another grid/chart/custom element via Trunk.

---

### 1️⃣ Drop the JS file into `static/js/`

```js
class MyNewComponent extends HTMLElement {
  // lifecycle logic here
}

customElements.define("my-new-component", MyNewComponent);
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
    <my-new-component data={data} config={config}></my-new-component>
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

---

### 🐚 Bash Scripts for JS Library Fetching

We used simple shell scripts to automate fetching large JS libraries (like AG Grid) from `npm` without checking them into Git.

| Script                        | Purpose                                | Notes |
|--------------------------------|----------------------------------------|-------|
| `scripts/fetch-ag-grid.sh`     | Downloads AG Grid Community module     | Cleans old files, uses `npm install`, copies only needed files to `static/vendor/ag-grid-community/` |
| `scripts/fetch-ag-grid-enterprise.sh` | Same but for AG Grid Enterprise     | Requires `npm`; avoids bloating Git repo; reproducible fetch process |


#### Why?

- Keeps Git repo clean.
- Avoids checking in `node_modules/` or vendor JS code.
- Ensures repeatable builds — just re-run the script if AG Grid releases a new version.
- `npm` is *only* a build-time fetch tool — not required for runtime.

---

## MarkDown to Static Page

This project supports fully static markdown-driven pages with metadata — great for documentation or blog-style content.

Markdown files are placed in:

```
static/content/*.md
```

Each file uses YAML frontmatter for metadata and on build the page and content is automatically added to the web app.

---

## 🧰 About Trunk

**Trunk** is the 🔧 build tool for Rust WASM web apps. It handles:

- 🦀 Building Rust → WASM
- 📦 Bundling assets (JS, CSS, images)
- 🌐 Live reloading + serving
- 🔒 Subresource integrity (SRI) support
- 🧩 Minimal config, big results

Official docs: [trunkrs.dev](https://trunkrs.dev/)  
Key reference: [Assets page](https://trunkrs.dev/assets/) — bookmark this one!

> 💡 *Tip:* Always cross-check `index.html` asset paths with the generated `dist/` tree!

---

### 🛠️ Common CLI Commands

```bash
# Start dev server with hot reload
trunk serve --open

# One-off build
trunk build

# Verbose output (debugging)
trunk build -v

# Clean slate
trunk clean
```

---

## 📝 Notes for the Future

- 🧳 **Trunk is not a bundler** — it doesn't tree-shake JS or optimize dependencies.  
  For larger JS needs, consider [Vite](https://vitejs.dev/) or [Webpack](https://webpack.js.org/).

- 🌒 Shadow DOM: This project avoids Shadow DOM for simplicity — AG Grid and most external libs don’t use it. If you're building styled components from scratch, Shadow DOM may be worth exploring.
