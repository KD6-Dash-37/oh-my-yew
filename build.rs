/// build.rs

use glob::glob;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct StaticPageMeta {
    title: String,
    date: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StaticPage {
    meta: StaticPageMeta,
    html_output: String,
    route: String,  // e.g. "/docs/styling/styling_css"
    section: String, // e.g. "docs"
    slug: String,    // e.g. "styling_css"
    parent: Option<String>
}

fn main() {
    println!("cargo:rerun-if-changed=static/pages/");

    let mut pages = Vec::new();

    for entry in glob("static/pages/**/*md").expect(
        "Failed to read glob pattern"
    ) {
        let path = entry.expect("Invalid path");

        // Read and split file
        let content = fs::read_to_string(&path).expect("Could not read file");
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() != 3 {
            panic!("Missing fontmatter in {}", path.display());
        }

        // Parse fontmatter and markdown
        let meta: StaticPageMeta = serde_yaml::from_str(parts[1])
            .expect("Invalid frontmatter format");
        let markdown = parts[2].trim();

        let parser = Parser::new_ext(markdown, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Build route and metadata
        let relative_path = path.strip_prefix("static/pages").unwrap();
        let slug = path.file_stem().unwrap().to_str().unwrap().to_string();

        let route = {
            let mut route = String::from("/");
            route.push_str(&relative_path.with_extension("").to_string_lossy().replace("\\", "/"));
            route
        };

        let section = relative_path.iter().next().unwrap().to_str().unwrap().to_string();

        // Determine parent route (if any)
        let parent = {
            let comps: Vec<_> = route.split("/")
                .filter(|s| !s.is_empty())
                .collect();
            if comps.len() > 2 {
                Some(format!("/{}/{}", comps[0], comps[1]))
            } else {
                None
            }
        };

        pages.push(StaticPage {
            meta,
            html_output,
            route,
            section,
            slug,
            parent
        });
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        fs::create_dir_all(&out_dir).unwrap();

    let json = serde_json::to_string_pretty(&pages).expect("Failed to serialise JSON");
    fs::write(out_dir.join("pages.json"), json).expect("Failed to write pages.json");

    println!("📦 Built static page index with {} pages", pages.len());
}