use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section>
            <h1>{"Welcome to my Yew App"}</h1>
            <p>{ "This is the landing page. Choose a section from the sidebar." }</p>
        </section>
    }
}