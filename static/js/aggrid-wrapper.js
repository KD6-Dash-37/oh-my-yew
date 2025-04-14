class AgGridTable extends HTMLElement {
    constructor() {
        super();

        this.style.display = "block";
        this.style.height = "400px";

        this.gridDiv = document.createElement("div");
        this.gridDiv.style.height = "100%";
        this.gridDiv.style.width = "100%";
        this.gridDiv.classList.add("ag-theme-alpine");

        this.appendChild(this.gridDiv);
    }

    connectedCallback() {
        const dataAttr = this.getAttribute("data");
        const columnsAttr = this.getAttribute("columns");

        const data = dataAttr ? JSON.parse(dataAttr) : [];
        const columns = columnsAttr ? JSON.parse(columnsAttr) : [];

        const gridOptions = {
            rowData: data,
            columnDefs: columns,
            sideBar: true, // now this works 🎉
            theme: 'legacy', // prevent warning about theming conflict
        };

        this.gridApi = agGrid.createGrid(this.gridDiv, gridOptions);
    }

    disconnectedCallback() {
        if (this.gridApi?.destroy) {
            this.gridApi.destroy();
        }
    }
}

customElements.define("ag-grid-table", AgGridTable);
