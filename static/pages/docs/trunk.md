---
title: "Trunk"
date: "2025-04-15"
tags: ["trunk", "bindings", "js"]
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

### 🧷 Asset Conventions & Tips

- `data-trunk` attributes in `index.html` are **your primary asset config** — no `Trunk.toml` needed in most cases.
- Use `data-target-path` to control where things land inside `dist/`.
  ```html
  <script data-trunk src="static/js/foo.js" type="module" data-target-path="js"></script>
  ```
- Use `rel="copy-file"` or `rel="copy-dir"` to include vendor files like UMD/CSS manually.

> 📂 *You control what gets bundled by how you link it — not by file placement alone.*

---

### 🎯 Pro Tips

- ⚠️ Trunk does **not** currently support watching arbitrary Rust files like `build.rs`.  
  Use `println!("cargo:rerun-if-changed=...")` to force rebuilds on static content changes.
  
- 🕵️ If you're not seeing something in the browser:
  - Check the `dist/` folder — does your file exist there?
  - Check browser devtools → Network tab → look for 404s.
  - Check `trunk build -v` output for hints.

- 🪄 Trunk supports `.scss` and `.sass` out of the box if you need styling sugar — just link it with `rel="scss"` or `rel="sass"`.

---

### 🔗 Useful Trunk Config Snippets

If you need to customize, you can add a `Trunk.toml` file like this:

```toml
[build]
target = "index.html"

[watch]
# Explicit extra paths to watch (besides src and static/)
paths = ["static/pages/"]

# Ignore node_modules or generated output
ignore = ["static/vendor/"]
```

> 🧩 *You usually won’t need this unless you’re watching generated or deeply nested assets.*

---
