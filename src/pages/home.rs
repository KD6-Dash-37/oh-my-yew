// src/pages/home.rs

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::docs::load_docs;


#[function_component(Home)]
pub fn home() -> Html {
    let docs = load_docs();

    html! {
        <>
            <h1>{ "Home Page"}</h1>
            <nav>
                <ul>
                    <li>
                        <Link<Route> to={Route::Chart}>{"Chart Page"}</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Grid}>{"Grid Page"}</Link<Route>>
                    </li>
                    { for docs.iter().map(|doc| html! {
                        <li><a href={doc.meta.route.clone()}>{ &doc.meta.title }</a></li>
                    })}
                </ul>
            </nav>
        </>
    }
}
