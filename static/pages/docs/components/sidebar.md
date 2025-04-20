---
title: Sidebar
section: docs
slug: sidebar
date: 2025-04-19
tags: [sidebar, navigation, layout, yew]
---

# Sidebar Component Design

This page documents the general approach used to implement the sidebar navigation component in this app. The sidebar is dynamic and context-aware, switching between a full index for documentation pages and a compact app-style navigation on application pages.

---

## Purpose

The sidebar serves two roles:

1. **Full sidebar for Docs**  
   - Lists top-level documentation pages, with nested child links if present.
   - Toggable visibility with a ☰ menu button.
2. **Mini sidebar for App pages**  
   - Shows compact icon links to key application routes (e.g., home, plotly, grid).

---

## Behavior Summary

- When the current route starts with `/docs` or is `/`, the full sidebar opens by default.
- When on an app route (e.g. `/app/plotly`), a minimal sidebar with icons is shown instead.
- The full sidebar includes a button that toggles its visibility.

---

## Docs Sidebar

The docs navigation uses a flattened list of all pages with a `section == "docs"`. Pages are grouped by whether they are **top-level** (no parent) or **nested** (with a `parent` route set at build time).

```rust
let (parents, children): (Vec<_>, Vec<_>) = pages.iter()
    .filter(|p| p.section == "docs")
    .partition(|p| p.parent.is_none());
```

The sidebar renders parent links first, followed by a sublist of children, using `.filter()` to match by parent route.

---

## Mini Sidebar

The mini sidebar is always visible on app pages and includes icon links to:

- Home 🏠
- Plotly 📈
- AG Grid 📊

It is defined as a separate Yew component for clarity.

---

## Styling

Two key classes control the sidebar appearance:

- `.sidebar` (with optional `.open` or `.closed`)
- `.mini-sidebar` for compact layout

These are defined in `static/css/sidebar.css`.

---

## How to Modify

- To add new doc sections or pages, edit the markdown files in `static/pages/docs/...`.
- To change the app icons or labels, edit the `MiniSidebar` component.
- CSS layout and transitions live in `sidebar.css`.

---

This flexible layout allows for future expansion, like adding active highlighting or nested navigation trees.
```