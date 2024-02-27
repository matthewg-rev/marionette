class GraphWidget extends Widget {
    constructor (title, width, height) {
        super(title, width, height);


        this.container = this.element.appendChild(document.createElement('canvas'));
        this.container.id = 'graph-canvas';

        this.centerButton = this.element.appendChild(document.createElement('div'));
        this.centerButton.id = 'graph-center-button';
        this.centerButton.innerHTML = '󰆤';

        this.ctx = this.container.getContext('2d');
        this.graph = new Graph();

        this.binds['containerMouseDown'] = this.containerMouseDown.bind(this);
        this.binds['containerMouseMove'] = this.containerMouseMove.bind(this);
        this.binds['containerMouseUp'] = this.containerMouseUp.bind(this);
        this.binds['containerWheel'] = (e) => this.adjustZoom(-e.deltaY * this.camera.scrollSensitivity);
        this.binds['centerButtonClick'] = this.centerButtonClick.bind(this);
        
        this.camera = {
            x: this.container.style.width / 2,
            y: this.container.style.height / 2,

            zoom: 0.1,
            lastZoom: 0.1,
            maxZoom: 5,
            minZoom: 0.1,
            scrollSensitivity: 0.005,
            mouseSensitivity: 2
        }

        this.states = {
            updated: {flag: true},
            dragging: {flag: false, start: {x: 0, y: 0}},
        }

        this.onExpand['graph'] = () => {
            this.container.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
            this.centerButton.style.visibility = this.flags.expanded ? 'visible' : 'hidden';

            if (this.flags.expanded) {
                this.container.addEventListener('mousedown', this.binds.containerMouseDown);
                this.container.addEventListener('mousemove', this.binds.containerMouseMove);
                this.container.addEventListener('mouseup', this.binds.containerMouseUp);
                this.container.addEventListener('wheel', this.binds.containerWheel);
                this.centerButton.addEventListener('click', this.binds.centerButtonClick);
            } else {
                this.container.removeEventListener('mousedown', this.binds.containerMouseDown);
                this.container.removeEventListener('mousemove', this.binds.containerMouseMove);
                this.container.removeEventListener('mouseup', this.binds.containerMouseUp);
                this.container.removeEventListener('wheel', this.binds.containerWheel);
                this.centerButton.removeEventListener('click', this.binds.centerButtonClick);
            }
        }

        this.onMouseLeave['graph'] = (e) => {
            this.states.dragging.flag = false;
        }

        this.draw();
    }

    draw() {
        if (this.flags.expanded && !this.flags.dragging && !this.flags.moving && this.states.updated.flag) {
            this.states.updated.flag = false;
            this.container.width = window.innerWidth;
            this.container.height = window.innerHeight;

            this.ctx.translate(window.innerWidth / 2, window.innerHeight / 2);
            this.ctx.scale(this.camera.zoom, this.camera.zoom);
            this.ctx.translate(-window.innerWidth / 2 + this.camera.x, -window.innerHeight / 2 + this.camera.y);
            this.ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);

            if (this.graph.root) {
                this.graph.root.renderer.render(this, this.ctx);
            }
        }

        requestAnimationFrame(() => this.draw());
    }

    getLocationFromEvent(e) {
        if (e.touches && e.touches.length == 1) {
            return {x: e.touches[0].clientX, y: e.touches[0].clientY};
        } else if (e.clientX && e.clientY) {
            return {x: e.clientX, y: e.clientY};
        }
    }

    containerMouseDown(e) {
        let location = this.getLocationFromEvent(e);
        this.states.dragging.flag = true;
        this.states.dragging.start.x = location.x/this.camera.zoom - this.camera.x;
        this.states.dragging.start.y = location.y/this.camera.zoom - this.camera.y;
    }

    containerMouseMove(e) {
        if (this.states.dragging.flag) {
            let location = this.getLocationFromEvent(e);
            let ox = this.camera.x;
            let oy = this.camera.y;
            this.camera.x = location.x/this.camera.zoom - this.states.dragging.start.x;
            this.camera.y = location.y/this.camera.zoom - this.states.dragging.start.y;
            this.states.updated.flag = this.camera.x !== ox || this.camera.y !== oy;
        }
    }

    containerMouseUp(e) {
        this.states.dragging.flag = false;
        this.camera.lastZoom = this.camera.zoom;
    }

    centerButtonClick(e) {
        this.camera.x = window.innerWidth / 2;
        this.camera.y = window.innerHeight / 2;
        this.states.updated.flag = true;
    }

    adjustZoom(zoomAmount, zoomFactor) {
        if (!this.states.dragging.flag) {
            if (zoomAmount)
                this.camera.zoom += zoomAmount;
            else if (zoomFactor)
                this.camera.zoom = zoomFactor * this.camera.lastZoom;

            this.camera.zoom = Math.min(this.camera.zoom, this.camera.maxZoom);
            this.camera.zoom = Math.max(this.camera.zoom, this.camera.minZoom);
            this.states.updated.flag = this.camera.zoom !== this.camera.lastZoom;
        }
    }
}