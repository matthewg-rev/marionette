class GraphWidget extends Widget {
    constructor (title, width, height) {
        super(title, width, height);
        this.container = this.element.appendChild(document.createElement('div'));
        this.container.id = 'graph-container';

        this.onExpand['graph'] = () => {
            this.container.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
        }
    }
}