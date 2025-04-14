---
title: "Markdown to Static Pages"
date: "2025-04-15"
route: "/docs/markdown-to-static"
tags: ["markdown", "static-page"]
---


This project supports fully static markdown-driven pages with metadata — great for documentation or blog-style content.

Markdown files are placed in:

```
static/content/*.md
```

Each file uses YAML frontmatter for metadata:

```md
---
title: "Intro to WASM"
date: "2025-04-12"
route: "/docs/intro"
tags: ["wasm", "rust", "guide"]
---

# Hello, WASM

This is my first content page...
```

---

### How It Works

| Step | What | Notes |
|------|------|-------|
| `build.rs` | Parses `.md` files at compile time | Splits YAML frontmatter & markdown body |
| `pulldown-cmark` | Converts markdown → HTML | Injects this HTML into `docs.json` |
| Trunk Copy | No copy config required | `include_str!` embeds `docs.json` directly into WASM binary |
| `DocsPage` | Dynamic page routed by slug | Pulls from pre-loaded context, no HTTP fetch needed |

---

### Pros of This Approach

✅ All content is static & pre-rendered at compile time  
✅ No runtime fetch required — super fast  
✅ Markdown edits auto-hot-reload with:

```rust
println!("cargo:rerun-if-changed=static/content/");
```

✅ Clean routing with Yew:

```rust
#[derive(Routable)]
pub enum Route {
    #[at("/docs/:slug")]
    DocsPage { slug: String },
}
```

---

### Future Enhancements Ideas

- Tailwind styling for markdown content  
- Auto-generated sidebar nav for docs  
- Tag-based filters or search  
- Paginated blog posts  

Excellent call — here’s a clean "How to Add a New Content Page" section you can drop into the README:

---

## Adding a New Markdown Page

To add a new page to the site:

---

### 1. Create the Markdown File

In `static/content/`:

```bash
touch static/content/my-new-page.md
```

---

### 2. Add Frontmatter Metadata

This defines the page structure & routing:

```md
---
title: "My New Page"
date: "2025-04-14"
route: "/docs/my-new-page"
tags: ["example", "guide"]
---

# Hello World

This content will be rendered as HTML on your page.
```

---

### 3. Build + Serve

Trunk will automatically:

- Re-run `build.rs` (because of `cargo:rerun-if-changed`)
- Regenerate `docs.json`
- Rebuild your WASM binary with the new content embedded
- Hot reload the browser

```bash
trunk serve --open
```

---

### 4. Verify It’s Linked

If you’ve added the page correctly:

- It will show up in the Home page docs index
- It will render with markdown formatting
- It will be available at:

```
http://localhost:8080/docs/my-new-page
```

---

## Important Notes

| Step | Why It Matters |
|------|----------------|
| Frontmatter `route` | Defines the URL — must be unique & prefixed `/docs/` |
| `title` & `date` | Used for display in the docs index |
| `tags` | Available for future filters/search |
| Body Content | Markdown converted to HTML at build time via `pulldown-cmark` |

---

This keeps your app fully static, zero runtime fetches, and fast-loading.

---

