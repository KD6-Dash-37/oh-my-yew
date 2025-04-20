---
title: "Introduction"
date: "2025-04-01"
tags: ["getting-started", "overview"]
---


### 🧭 How the Site is Structured

This site combines two types of content:

- **📝 Static Pages** (Markdown-based):  
  Authored under `static/pages/`, converted to HTML at build time, and routed dynamically using the folder structure (e.g., `/docs/styling/styling_css`). Ideal for documentation, notes, and structured content.

- **🧩 Applications** (Interactive Components):  
  Rust/Yew/WASM apps located under `src/apps/`, each with their own route (e.g., `/apps/plotly`, `/apps/aggrid`). These demonstrate JS/WASM interop, charts, UI components, etc.

Both are listed in the sidebar, and routing is unified — no difference to the user. The layout adjusts depending on whether you’re viewing static content or running an interactive app.

> All pages and apps share the same layout and styling system, so you can mix static docs with live demos.
