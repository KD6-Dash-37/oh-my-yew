---
title: "Styling with CSS"
date: "2025-04-03"
tags: ["css"]
---

Absolutely! Here's a concise and practical reference section you can include in your docs for applying CSS with **Trunk** + **Rust/Yew/WASM**:

---

## 🎨 Applying CSS in Rust/Yew Projects

Yew doesn't use inline styles by default — instead, CSS is handled externally and referenced in HTML or applied via class names in your components.

---

### ✅ Trunk Setup (One-Time)

Trunk handles CSS files via `index.html`. Add this to the `<head>`:

```html
 <link 
    data-trunk 
    rel="css" 
    href="static/css/sidebar.css" 
/>
```

- Trunk will copy and hash the file into `/dist/`
- The stylesheet is automatically linked in the final build

> 🔁 No need to list it in `Trunk.toml` if you use `rel="css"` in `index.html`.

---

### 🧠 Using CSS in Rust Components

Use the `class` or `classes!()` macro:

```rust
html! {
    <div class="my-class">{"Hello!"}</div>
}
```

Or conditionally:

```rust
let is_active = true;
let class_name = if is_active {
    classes!("nav-item", "active")
} else {
    classes!("nav-item")
};

html! { <div class={class_name}>{ "Item" }</div> }
```

---

### ✍️ Style Organization Tips

- Store styles under: `static/css/`
- You can break out stylesheets per component (e.g., `sidebar.css`) and import them the same way.
- For global styles, stick to a single `styles.css` or reset file.

---

### 🧪 Dev Flow Reminder

- Edit `static/css/*.css`
- Trunk will hot-reload changes in the browser
- Use browser DevTools (`Elements` + `Styles`) to debug live

---

Want to add an example snippet or want help breaking out multiple stylesheets?
