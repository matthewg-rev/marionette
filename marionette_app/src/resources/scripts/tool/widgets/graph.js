class GraphWidget extends Widget {
    constructor (title, width, height) {
        super(title, width, height);

        this.container = this.element.appendChild(document.createElement('canvas'));
        this.container.id = 'graph-canvas';

        this.centerButton = this.element.appendChild(document.createElement('div'));
        this.centerButton.id = 'graph-center-button';
        this.centerButton.innerHTML = '󰆤';

        this.ctx = this.container.getContext('2d');

        this.renderer = new BoxRenderer();
        this.graph = Graph.fromJSON(Graph.testJSON());

        this.binds['containerMouseClick'] = this.containerMouseClick.bind(this);
        this.binds['containerMouseDown'] = this.containerMouseDown.bind(this);
        this.binds['containerMouseMove'] = this.containerMouseMove.bind(this);
        this.binds['containerMouseUp'] = this.containerMouseUp.bind(this);
        this.binds['containerWheel'] = this.containerMouseWheel.bind(this);
        this.binds['centerButtonClick'] = this.centerButtonClick.bind(this);
        this.binds['containerTouchStart'] = this.containerTouchStart.bind(this);
        this.binds['containerTouchEnd'] = this.containerTouchEnd.bind(this);
        this.binds['containerTouchMove'] = this.containerTouchMove.bind(this);
        
        this.camera = {
            x: this.container.style.width / 2,
            y: this.container.style.height / 2,

            zoom: 1,
            lastZoom: 1,
            maxZoom: 5,
            minZoom: 0.1,
            scrollSensitivity: 0.005,
            mouseSensitivity: 2
        }

        this.states = {
            selected: {vertex: null},
            trackpad: {flag: false},
            mouse: {flag: false},
            
            mouseDown: {flag: false},
            allowSelection: {flag: true},

            updated: {flag: true},
            dragging: {flag: false, start: {x: 0, y: 0}},
            pinching: {start: {distance: null}}
        }

        $(this.container).click(this.binds.containerMouseClick);

        this.onExpand['graph'] = () => {
            this.container.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
            this.centerButton.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
            
            this.centerButton.style.opacity = this.flags.expanded ? '1' : '0';
            this.centerButton.style.width = this.flags.expanded ? '40px' : '0px';
            this.centerButton.style.height = this.flags.expanded ? '40px' : '0px';

            if (this.flags.expanded) {
                this.container.addEventListener('mousedown', this.binds.containerMouseDown);
                this.container.addEventListener('mousemove', this.binds.containerMouseMove);
                this.container.addEventListener('mouseup', this.binds.containerMouseUp);
                this.container.addEventListener('mousewheel', this.binds.containerWheel);

                this.container.addEventListener('touchstart', this.binds.containerTouchStart);
                this.container.addEventListener('touchend', this.binds.containerTouchEnd);
                this.container.addEventListener('touchmove', this.binds.containerTouchMove);

                this.centerButton.addEventListener('click', this.binds.centerButtonClick);
            } else {
                this.container.removeEventListener('mousedown', this.binds.containerMouseDown);
                this.container.removeEventListener('mousemove', this.binds.containerMouseMove);
                this.container.removeEventListener('mouseup', this.binds.containerMouseUp);
                this.container.removeEventListener('mousewheel', this.binds.containerWheel);

                this.container.removeEventListener('touchstart', this.binds.containerTouchStart);
                this.container.removeEventListener('touchend', this.binds.containerTouchEnd);
                this.container.removeEventListener('touchmove', this.binds.containerTouchMove);
                
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
                this.renderer.preprocess(this.ctx, this.graph);
                this.renderer.render(this.ctx, this);
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

    getMousePos(canvas, e) {
        let rect = canvas.getBoundingClientRect();
        let scaleX = canvas.width / rect.width;
        let scaleY = canvas.height / rect.height;

        return {
            x: (e.clientX - rect.left) * scaleX,
            y: (e.clientY - rect.top) * scaleY
        };
    }

    canvasCoordinates(matrix, pos) {
        let iMatrix = matrix.invertSelf();
        let x = pos.x * iMatrix.a + pos.y * iMatrix.c + iMatrix.e;
        let y = pos.x * iMatrix.b + pos.y * iMatrix.d + iMatrix.f;
        return {x: x, y: y};
    }

    containerMouseClick(e) {
        if (!this.states.allowSelection.flag) {
            return;
        }

        // TODO: offload the Matrix 
        let canvasMouseLocation = this.getMousePos(this.container, e);
        let transformMatrix = this.ctx.getTransform();
        let canvasSpace = this.canvasCoordinates(transformMatrix, canvasMouseLocation);

        // iterate through the renderer's vertices and check if the click was inside any of them
        if (this.graph) {
            this.states.selected.vertex = this.renderer.select(this.ctx, {x: canvasSpace.x, y: canvasSpace.y});
            this.states.updated.flag = true;
        }
    }

    containerMouseDown(e) {
        let location = this.getLocationFromEvent(e);
        this.states.mouseDown.flag = true;
        this.states.dragging.start.x = location.x/this.camera.zoom - this.camera.x;
        this.states.dragging.start.y = location.y/this.camera.zoom - this.camera.y;
    }

    containerMouseMove(e) {
        if (this.states.mouseDown.flag) {
            this.states.dragging.flag = true;
            let location = this.getLocationFromEvent(e);
            let ox = this.camera.x;
            let oy = this.camera.y;
            this.camera.x = location.x/this.camera.zoom - this.states.dragging.start.x;
            this.camera.y = location.y/this.camera.zoom - this.states.dragging.start.y;

            this.states.updated.flag = this.camera.x !== ox || this.camera.y !== oy;
        }
    }

    containerMouseUp(e) {
        this.states.mouseDown.flag = false;
        if (this.states.dragging.flag) {
            this.states.dragging.flag = false;
            this.states.allowSelection.flag = false;
            setTimeout(() => this.states.allowSelection.flag = true, 0);
        }
        this.camera.lastZoom = this.camera.zoom;
    }

    // if im on my thinkpad laptop the way to zoom is different
    // draw your two fingers downwards or up, not pinch like on
    // touch screens
    containerMouseWheel(e) {
        var isTouchPad = e.wheelDeltaY ? e.wheelDeltaY === -3 * e.deltaY : e.deltaMode === 0;
        this.states.trackpad.flag = isTouchPad;
        this.states.mouse.flag = !isTouchPad;

        this.adjustZoom(-e.deltaY * this.camera.scrollSensitivity);
    }

    containerHandleTouch(e, touchHandler) {
        if (e.touches.length == 1) {
            touchHandler(e)
        } else if (e.type == "touchmove" && e.touches.length == 2) {
            this.states.dragging.flag = false;
            this.containerHandlePinch(e);
        }
    }

    containerHandlePinch(e) {
        e.preventDefault();

        let touch1 = { x: e.touches[0].clientX, y: e.touches[0].clientY };
        let touch2 = { x: e.touches[1].clientX, y: e.touches[1].clientY };

        let distance = (touch1.x - touch2.x)**2 + (touch1.y - touch2.y)**2;
        if (this.states.pinching.start.distance == null) {
            this.states.pinching.start.distance = distance;
        } else {
            this.adjustZoom(0, distance / this.states.pinching.start.distance);
        }
    }

    containerTouchStart(e) {
        this.containerHandleTouch(e, this.binds.containerMouseDown);
    }

    containerTouchEnd(e) {
        this.containerHandleTouch(e, this.binds.containerMouseUp);
    }

    containerTouchMove(e) {
        this.containerHandleTouch(e, this.binds.containerMouseMove);
    }

    centerButtonClick(e) {
        this.camera.x = 0;
        this.camera.y = 0;
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