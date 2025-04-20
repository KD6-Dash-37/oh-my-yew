// src/main.rs

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod route;
pub mod home;

pub mod apps;
pub mod components;

use pages::load_static_pages;
use route::{Route, switch};
use components::SidebarSelector;


#[function_component(Main)]
fn main_component() -> Html {
    let pages = use_state(load_static_pages);

    html! {
        <ContextProvider<Vec<pages::StaticPage>> context={(*pages).clone()}>
            <BrowserRouter>
                <div class="layout">
                    <SidebarSelector />
                    <main class="main-content">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </BrowserRouter>
        </ContextProvider<Vec<pages::StaticPage>>>
    }
}


fn main() {
    yew::Renderer::<Main>::new().render();
}
