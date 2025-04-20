---
title: "JS Libraries"
date: "2025-04-20"
tags: ["bash", "npm", "scripts"]
---


### 🐚 Bash Scripts for JS Library Fetching

This project uses small shell scripts to automate fetching large JS libraries — like AG Grid — directly from `npm`. These scripts are used during setup and build time to avoid committing third-party assets into the repository.

---

#### 📜 Overview

| Script                             | Purpose                                 | Notes                                                                 |
|------------------------------------|-----------------------------------------|-----------------------------------------------------------------------|
| `scripts/fetch-ag-grid.sh`         | Fetches AG Grid Community module        | Cleans previous copies, installs via `npm`, copies only required UMD/CSS files |
| `scripts/fetch-ag-grid-enterprise.sh` | Fetches AG Grid Enterprise module     | Same structure as Community; includes enterprise-specific styles and bundles |

---

#### 🧠 Why Use Bash Scripts?

- ✅ Keeps the Git repository clean and lean  
  → No `node_modules/`, no giant `vendor/` folders checked into version control.

- 🔁 Enables reproducible fetches  
  → Just re-run the script to update to a new AG Grid version.

- 🧰 Uses `npm` *only* as a build-time tool  
  → There's **no JavaScript runtime dependency** in production.

- 🗃️ Output is copied to:
  ```
  static/vendor/ag-grid-community/
  static/vendor/ag-grid-enterprise/
  ```

---

#### 🏗️ How to Run

```bash
./scripts/fetch-ag-grid.sh
./scripts/fetch-ag-grid-enterprise.sh
```

> Note: You’ll need `npm` installed locally for these scripts to work.

---

#### 📦 Tip: Re-fetch When Needed

These scripts are intended to be re-run manually if AG Grid releases a new version or if changes are made to the directory layout.

You can safely add this to your internal `README.md` or project setup checklist:

```bash
# One-time or version update
bash scripts/fetch-ag-grid.sh
bash scripts/fetch-ag-grid-enterprise.sh
```

---

That’s it — quick, reproducible, no `node_modules` in sight.