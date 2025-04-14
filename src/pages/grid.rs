// src/pages/grid.rs

use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;


use crate::app::Route;

#[function_component(Grid)]
pub fn grid() -> Html {
    let data = r#"[{"name":"BTC","price":50000},{"name":"ETH","price":3000}]"#;
    let columns = r#"[{"field":"name"},{"field":"price"}]"#;

    html! {
        <>
            <h1>{"AG Grid Table"}</h1>
            <Link<Route> to={Route::Home}>{ "Back to Home"}</Link<Route>>
            <div>
                {
                    VNode::from_html_unchecked(AttrValue::from(format!(
                        r#"<ag-grid-table data='{}' columns='{}'></ag-grid-table>"#,
                        data,
                        columns
                        ))
                    )
                }
            </div>
        </>
    }
}