// src/apps/ag_grid/mod.rs

use yew::prelude::*;
use yew::virtual_dom::VNode;


#[function_component(AgGrid)]
pub fn grid() -> Html {
    let data = r#"[{"name":"BTC","price":50000},{"name":"ETH","price":3000}]"#;
    let columns = r#"[{"field":"name"},{"field":"price"}]"#;

    html! {
        <>
            <h1>{"AG Grid Table"}</h1>
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