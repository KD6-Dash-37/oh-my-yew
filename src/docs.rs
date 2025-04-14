// src/docs.rs
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::app::Route;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DocMeta {
    pub title: String,
    pub date: String,
    pub route: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Doc {
    pub meta: DocMeta,
    pub html_output: String,
}

const DOCS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/docs.json"));

pub fn load_docs() -> Vec<Doc> {
    serde_json::from_str(DOCS_JSON).expect(
        "Docs context not found — make sure ContextProvider is set in App"
    )
}

#[derive(Properties, PartialEq)]
pub struct DocsPageProps {
    pub slug: String,
}

#[function_component(DocsPage)]
pub fn docs_page(props: &DocsPageProps) -> Html {
    let docs = use_context::<Vec<Doc>>()
        .expect("Docs context not found");

    let doc = docs.iter()
        .find(|d| d.meta.route == format!("/docs/{}", props.slug))
        .cloned();

    html! {
        <>
            <Link<Route> to={Route::Home}>{" Back to Home"}</Link<Route>>

            if let Some(doc) = doc {
                <article>
                    <h1>{ &doc.meta.title }</h1>
                    <p><small>{ &doc.meta.date }</small></p>
                    <div>
                        { yew::virtual_dom::VNode::from_html_unchecked(AttrValue::from(doc.html_output)) }
                    </div>
                </article>
            } else {
                <p>{ "Document not found" }</p>
            }
        </>
    }
}