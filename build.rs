// build.rs

use glob::glob;
use std::{env, fs, path::Path};
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Options, html};

#[derive(Serialize, Deserialize, Debug)]
struct DocMeta {
    title: String,
    date: String,
    route: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Doc {
    meta: DocMeta,
    html_output: String,
}

fn main() {
    // Ensure build.rs reruns if any markdown content changes
    println!("cargo:rerun-if-changed=static/content/");

    let mut docs: Vec<Doc> = Vec::new();

    for entry in glob("static/content/*.md")
        .expect("Failed to read glob pattern") {
            let path = entry.expect("Invalid path");
            let content = fs::read_to_string(&path).expect("Could not read file");

            let parts: Vec<&str> = content.splitn(3,  "---").collect();
            if parts.len() != 3 {
                panic!("Missing frontmatter in {}", path.display());
            }

            let meta: DocMeta = serde_yaml::from_str(parts[1]).expect("Failed to parse frontmatter");
            let markdown_input = parts[2].trim();
            let parser = Parser::new_ext(markdown_input, Options::all());
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            docs.push(Doc { meta, html_output, });
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("docs.json");
    let json = serde_json::to_string_pretty(&docs).unwrap();
    fs::write(dest_path, json).expect("Failed to write docs.json")
}