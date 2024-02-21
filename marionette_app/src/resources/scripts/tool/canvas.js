class Canvas {
    constructor() {
        this.element = document.getElementsByClassName('canvas')[0];
        this.widgets = [];
        this.grid_size = 20;
    }

    addWidget(widget) {
        this.widgets.push(widget);
        widget.setCanvas(this);
    }

    removeWidget(widget) {
        this.widgets = this.widgets.filter(w => w !== widget);
    }
}