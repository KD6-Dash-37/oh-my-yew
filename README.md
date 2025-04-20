# 🚀 Oh My Yew

*A learn-as-you-go project exploring Rust + WASM with the [Yew](https://yew.rs/) framework — documenting each piece for future reference.*

---

### ✅ Features So Far

- 🔧 **Yew Pages & Routing**  
  App pages built with idiomatic Rust components using `yew-router`.

- 📚 **Static Content System**  
  Markdown files in [`static/pages/`](./static/pages) auto-indexed at compile time via `build.rs`.

- 🧠 **Hierarchical Sidebar Navigation**  
  Auto-generated from directory structure — supports parent/child pages.

- 💅 **Scoped CSS with Trunk**  
  Simple class-based styling via linked `.css` files; no bundler needed.

- 🔄 **JS Interop (2 Patterns)**  
  - **WASM Bindings**: Native Rust control of JS (e.g., Plotly chart)  
  - **Web Components**: For large JS libs (e.g., AG Grid Enterprise)

- 🐚 **Scripted JS Library Fetching**  
  Bash scripts pull assets from `npm` at build-time — no `node_modules/` in Git.

---

### 📖 Documentation

Full in-app docs live inside the app itself — see:  
[`static/pages/docs/`](./static/pages/docs)

Pages include guides for:

- 🏗️ Page + route generation  
- 🧩 JS/Web Component integration  
- 🎨 CSS via Trunk  
- 🐚 Bash asset fetchers  
- 🧱 Sidebar logic  
- 🪄 Tips + gotchas  

---

### 🔮 Roadmap / Demos in Progress

Curious what’s next?  
Check out the [📋 GitHub Issues](https://github.com/KD6-Dash-37/oh-my-yew/issues) for upcoming components and interactive demos

---

### 🧪 Getting Started

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
# optional: fetch AG Grid assets
./scripts/fetch-ag-grid.sh
./scripts/fetch-ag-grid-enterprise.sh

trunk serve --open
```
