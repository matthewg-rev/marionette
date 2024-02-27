class Canvas {
    constructor() {
        this.element = document.getElementsByClassName('canvas')[0];
        this.widgets = [];
        this.grid_size = 20;
    }

    addWidget(widget) {
        this.widgets.push(widget);
        widget.setCanvas(this);
        this.widgetFocused(widget);
    }

    highestZIndexWidget() {
        let max = 0;
        let maxWidget = null;
        this.widgets.forEach(w => {
            if (w.element.style.zIndex > max) {
                max = w.element.style.zIndex;
                maxWidget = w;
            }
        });

        if (maxWidget === null && this.widgets.length > 0) {
            maxWidget = this.widgets[0];
        }

        return [maxWidget, max];
    }

    widgetFocused(widget) {
        const indexes = this.widgets.map(w => w.element.style.zIndex).filter(z => z > widget.element.style.zIndex);
        indexes.forEach(z => {
            const w = this.widgets.find(w => w.element.style.zIndex === z);
            w.element.style.zIndex = z - 1;
        });

        widget.element.style.zIndex = this.widgets.length;
    }

    removeWidget(widget) {
        this.widgets = this.widgets.filter(w => w !== widget);
        let [maxWidget, max] = this.highestZIndexWidget();
        if (maxWidget !== null) {
            this.widgetFocused(maxWidget);
        }
        widget.element.remove();
    }
}