// src/route.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::StaticPageView;
use crate::apps::{PlotlyChart, AgGrid};
use crate::home::Home;


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:section/:slug")]
    StaticPage { section: String, slug: String },
    #[at("/app/plotly")]
    PlotlyApp,
    #[at("/app/ag-grid")]
    AgGridApp
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::StaticPage { section, slug } => {
            html! { <StaticPageView section={section} slug={slug} />}
        },
        Route::PlotlyApp => html! { <PlotlyChart /> },
        Route::AgGridApp => html! { <AgGrid /> }
    }
}