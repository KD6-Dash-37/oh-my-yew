---
title: "Static Content System"
date: 2025-04-20
tags: ["docs", "structure", "markdown"]
---

## Overview

This section documents the static markdown system that powers the docs section of the app. Each markdown file becomes a rendered page, and the directory structure defines hierarchy and routing automatically.

## 🆕 Adding a New Page

1. Add a new `.md` file under `static/pages/SECTION/`.
2. Add required frontmatter (see below).
3. (Optional) To create a parent page with children:
   - Create a file: `static/pages/SECTION/PARENT.md`
   - Create a folder: `static/pages/SECTION/PARENT/`
   - Add children pages inside this folder.

You do **not** need to register routes or update an index manually — the build script handles it.

---

## 📁 File Structure & Routing

The markdown structure under `static/pages/` defines the site layout. Each file becomes a page. Nested directories define child pages.

### Example

```bash
static/pages/
├── docs/
│   ├── introduction.md           → /docs/introduction
│   ├── styling.md                → /docs/styling
│   └── styling/
│       ├── styling_css.md       → /docs/styling/styling_css
│       └── styling_tailwind.md  → /docs/styling/styling_tailwind
├── blog/
│   └── 20250419.md              → /blog/20250419
```bash


- If a markdown file has a matching directory (e.g. `styling.md` + `styling/`), the files inside are treated as **child pages**.
- Pages are grouped and ordered within their section (e.g. `docs`, `blog`).

---

## 🧠 Frontmatter Requirements

Each page **must** start with YAML frontmatter:

```yaml
---
title: "Page Title"
date: "2025-04-19"
tags: ["optional", "tags"]
---
```


### Field Reference

| Field   | Required | Description                              |
|---------|----------|------------------------------------------|
| `title` | ✅        | Displayed in the sidebar and header      |
| `date`  | ✅        | Used for sorting, shown in article       |
| `tags`  | ❌        | Optional categorization / search metadata|

The `slug`, `section`, `route`, and `parent` fields are computed automatically by `build.rs`.

---

## 🛠️ How It Works

- On `cargo build`, the `build.rs` script:
  - Scans `static/pages/**/*.md`
  - Extracts frontmatter and body
  - Converts body to HTML using `pulldown-cmark`
  - Builds a JSON index of all pages (`OUT_DIR/pages.json`)
- The app loads this JSON at runtime and renders:
  - A nested sidebar navigation
  - Routes based on `/:section/:slug`
  - Page content in `<article>`

---

## 💡 Tips

- File names become slugs (e.g. `styling_css.md` → `styling_css`)
- Avoid `_underscore` prefixes unless you want to skip the page (draft support coming soon)
- Keep frontmatter minimal — everything else is derived

---

## 📍 Example Page

```md
---
title: "Styling with Tailwind"
date: "2025-04-20"
tags: ["styling", "tailwind", "css"]
---

## Introduction

Tailwind is a utility-first CSS framework...
