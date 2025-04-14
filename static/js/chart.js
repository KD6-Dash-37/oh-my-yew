// js/chart.js

export function renderChart(id, data, layout) {    
    // TODO remove this const and prepare the data in Rust instead
    const plainData = data.map(obj => Object.fromEntries(obj));
    Plotly.newPlot(id, plainData, layout)
}