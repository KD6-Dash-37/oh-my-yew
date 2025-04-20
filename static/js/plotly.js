// static/js/chart.js

export function renderChart(id, data, layout) {
    const chartDiv = document.getElementById(id);
    if (!chartDiv) return;

    const plainData = data.map(obj => Object.fromEntries(obj));

    Plotly.newPlot(chartDiv, plainData, layout).then(() => {
        // Resize once after initial render (e.g., after sidebar transition)
        setTimeout(() => {
            Plotly.Plots.resize(chartDiv);
        }, 50);

        // Add global resize listener (only once per id)
        if (!chartDiv._hasResizeListener) {
            window.addEventListener('resize', () => {
                Plotly.Plots.resize(chartDiv);
            });
            chartDiv._hasResizeListener = true; // prevent multiple listeners
        }
    });
}
