// src/apps/plotly/mod.rs

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;
use serde_json::json; 


#[wasm_bindgen(module = "/static/js/plotly.js")]
extern "C" {
    #[wasm_bindgen(js_name = "renderChart")]
    pub fn render_chart(id: &str, data: JsValue, layout: JsValue);
}

fn get_chart_data_for_instrument(instrument: &str) -> (Vec<f64>, Vec<f64>) {
    match instrument {
        "BTC-PERPETUAL" => (
            vec![1.0, 2.0, 3.0, 4.0],
            vec![100.0, 120.0, 90.0, 110.0],
        ),
        "ETH-PERPETUAL" => (
            vec![1.0, 2.0, 3.0, 4.0],
            vec![50.0, 60.0, 55.0, 65.0],
        ),
        _ => (
            vec![1.0, 2.0, 3.0, 4.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ),
    }
}


#[function_component(PlotlyChart)]
pub fn chart() -> Html {
    let options = vec![
        "BTC-PERPETUAL",
        "ETH-PERPETUAL",
    ];
    
    let selected = use_state(|| options[0].to_string());

    let on_change = {
        let selected = selected.clone();
        Callback::from(move |event: Event| {
            let select = event.target_dyn_into::<web_sys:: HtmlSelectElement>()
                .expect("Event target should be a select element");
            selected.set(select.value());
        }) 
    };

    let chart_id = "chart-div";

    let html = html! {
        <>
            <h1>{ "Chart" }</h1>
            // <Link<Route> to={Route::Home}>{ "Back to Home"}</Link<Route>>

            <div>
                <label>{"Instrument"}</label>
                <select onchange={on_change}>
                    { for options.iter().map(|opt| html! {
                        <option value={opt.to_string()} selected={*selected == *opt}>{ opt }</option>
                    })}
                </select>
            </div>

            <div id={chart_id}></div>

            <p>{ format!("Selected: {}", *selected) }</p>
        </>
    };
    {
        let selected = selected.clone();
        use_effect(move || {
            let (x, y) = get_chart_data_for_instrument(selected.as_str());
            let data = json!([
                {
                    "x": x,
                    "y": y,
                    "type": "scatter",
                    "mode": "lines+markers",
                    "name": selected.as_str(),
                }
            ]);
        
            let layout = json!({
                "title": format!("Chart for {}", *selected),
                "xaxis": { "title": "Time" },
                "yaxis": { "title": "Price" }
            });
        
            render_chart(
                chart_id,
                to_value(&data).unwrap(),
                to_value(&layout).unwrap()
            );
            || ()
        });
    }
    html
}

