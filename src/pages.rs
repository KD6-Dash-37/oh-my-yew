// src/pages.rs

use serde::{Deserialize, Serialize};
use yew::prelude::*;


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StaticPageMeta {
    pub title: String,
    pub date: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StaticPage {
    pub meta: StaticPageMeta,
    pub html_output: String,
    pub route: String,
    pub section: String,
    pub slug: String,
    pub parent: Option<String>
}

const PAGES_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/pages.json"));

pub fn load_static_pages() -> Vec<StaticPage> {
    serde_json::from_str(PAGES_JSON)
        .expect("Failed to parse pages.json -- was build.rs run?")
}


// ----------- Page Viewer -------------

#[derive(Properties, PartialEq)]
pub struct StaticPageProps {
    pub section: String,
    pub slug: String,
}

#[function_component(StaticPageView)]
pub fn static_page_view(props: &StaticPageProps) -> Html {
    let pages = use_context::<Vec<StaticPage>>()
        .expect("Static pages context not found");

    let current = pages.iter()
        .find(|p| p.section == props.section && p.slug == props.slug)
        .cloned();

    html! {
        <>
        {
            if let Some(page) = current {
                html! {
                    <article>
                        <h1>{ &page.meta.title }</h1>
                        if let Some(date) = &page.meta.date {
                            <p><small>{ date }</small></p>
                        }
                        <div>
                        { yew::virtual_dom::VNode::from_html_unchecked(AttrValue::from(page.html_output)) }
                        </div>
                    </article>
                }
            } else {
                html! {<p>{ "Page not found." }</p>}
            }
        }
        </>
    }
}