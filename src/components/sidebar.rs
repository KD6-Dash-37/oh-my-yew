// src/components/sidebar.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::pages::StaticPage;

#[function_component(SidebarSelector)]
pub fn sidebar_selector() -> Html {
    let location = use_location().expect("No location available");
    let path = location.path();

    if path.starts_with("/app") {
        html! { <MiniSidebar /> }
    } else {
        html! { <Sidebar /> }
    }
}

#[function_component(Sidebar)]
fn sidebar() -> Html {
    let location = use_location().expect("No location available");
    let path = location.path();
    let default_open = path.starts_with("/docs") || path == "/";

    let pages = use_context::<Vec<StaticPage>>().expect("Pages context missing");
    let is_open = use_state(move || default_open);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    let sidebar_classes = if *is_open {
        classes!("sidebar", "open")
    } else {
        classes!("sidebar", "closed")
    };

    let (parents, children): (Vec<_>, Vec<_>) = pages.iter()
        .filter(|p| p.section == "docs")
        .partition(|p| p.parent.is_none());


    html! {
        <>
            <button class="menu-button" onclick={toggle}>{ "☰" }</button>

            <aside class={sidebar_classes}>
                <nav>
                    <ul>
                    { for parents.iter().map(|parent| html! {
                        <>
                            <li>
                                <Link<Route> to={Route::StaticPage {
                                    section: parent.section.clone(),
                                    slug: parent.slug.clone()
                                }}>
                                    { &parent.meta.title }
                                </Link<Route>>
                            </li>
                            <ul class="child-list">
                                { for children.iter()
                                    .filter(|c| c.parent.as_deref() == Some(&parent.route))
                                    .map(|child| html! {
                                        <li>
                                            <Link<Route> to={Route::StaticPage {
                                                section: child.section.clone(),
                                                slug: child.slug.clone()
                                            }}>
                                                { &child.meta.title }
                                            </Link<Route>>
                                        </li>
                                    })
                                }
                            </ul>
                        </>
                    })}
                    
                    </ul>
                    <h3>{ "Apps" }</h3>
                    <ul>
                        <li><Link<Route> to={Route::PlotlyApp}>{ "Plotly" }</Link<Route>></li>
                        <li><Link<Route> to={Route::AgGridApp}>{ "AG Grid" }</Link<Route>></li>
                    </ul>
                
                </nav>
            </aside>
        </>
    }
}

#[function_component(MiniSidebar)]
fn mini_sidebar() -> Html {
    html! {
        <aside class="mini-sidebar">
            <nav>
                <ul>
                    <li>
                        <Link<Route> to={Route::Home} classes="icon-link">{ "🏠" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::PlotlyApp} classes="icon-link">{ "📈" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::AgGridApp} classes="icon-link">{ "📊" }</Link<Route>>
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
