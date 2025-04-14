// src/app.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, chart::Chart, grid::Grid};
use crate::docs::{load_docs, DocsPage, Doc};


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/chart")]
    Chart,
    #[at("/grid")]
    Grid,
    #[at("/docs/:slug")]
    DocsPage {slug: String},
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties =();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let docs = load_docs();
        html! {
            <BrowserRouter>
                <ContextProvider<Vec<Doc>> context={docs}>
                    <Switch<Route> render={switch} />
                </ContextProvider<Vec<Doc>>>
            </BrowserRouter>
        }
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::Chart => html! { <Chart /> },
        Route::Grid => html! {<Grid />},
        Route::DocsPage { slug} => html! {<DocsPage slug={slug} />},
    }
}
