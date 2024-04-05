class Widget {
    constructor(title, width, height) {
        let toolBar = document.getElementById('toolbar');
        this.startY = toolBar.clientHeight + 1;

        this.height = height;
        this.width = width;

        {
            this.element = document.createElement('div');
            this.element.id = 'widget';
            this.element.style.height = '20px';
            this.element.style.width = this.width + 'px';
            this.element.style.position = 'absolute';
            this.element.style.left = '0px';
            this.element.style.top = (toolBar.clientHeight + 1) + 'px';
            
            this.header = this.element.appendChild(document.createElement('div'));
            this.header.id = 'header';

            this.title = this.header.appendChild(document.createElement('div'));
            this.title.id = 'title';
            this.title.innerHTML = title;

            this.drop = this.header.appendChild(document.createElement('div'));
            this.drop.id = 'drop';
            this.drop.innerHTML = '󰅀';

            this.close = this.header.appendChild(document.createElement('div'));
            this.close.id = 'close';
            this.close.innerHTML = '';

            this.drag = this.element.appendChild(document.createElement('div'));
            this.drag.id = 'drag';
            this.drag.innerHTML = '󰑝';
        }

        this.binds = {
            headerMouseDown: this.headerMouseDown.bind(this),
            headerMouseMove: this.headerMouseMove.bind(this),
            headerMouseUp: this.headerMouseUp.bind(this),

            dragMouseDown: this.dragMouseDown.bind(this),
            dragMouseMove: this.dragMouseMove.bind(this),
            dragMouseUp: this.dragMouseUp.bind(this),

            dropClick: this.dropClick.bind(this),
            closeClick: this.closeClick.bind(this)
        }

        this.onExpand = {};
        this.onMouseLeave = {};

        this.positionInfo = {
            movement: {
                current: { x: 0, y: 25 },
                start: { width: this.element.offsetLeft, y: this.startY },
            },
            resizing: {
                currentSize: { width: this.width, height: this.height + 1 },
                current: { x: 0, y: this.startY },
                start: { x: this.element.offsetLeft, y: this.startY }
            },
            oldZIndex: 0
        }

        console.log(toolBar.clientHeight + 1);

        this.flags = {
            hasBindedEvents: false,
            expanded: false,
            dragging: false,
            moving: false
        }

        this.header.addEventListener('mousedown', this.binds.headerMouseDown);
        this.drop.addEventListener('click', this.binds.dropClick);
        this.close.addEventListener('click', this.binds.closeClick);
        this.element.addEventListener('mouseleave', (e) => {
            for (const key in this.onMouseLeave) {
                this.onMouseLeave[key](e);
            }
        });
    }

    closeClick(e) {
        for (const child of this.element.children) {
            if (child.id === 'header') continue;
            child.style.transition = 'all 0.5s ease-in-out';
            child.style.opacity = '0';
        }

        setTimeout(() => {
            for (const child of this.header.children) {
                child.style.transition = 'all 0.25s ease-in-out';
                child.style.opacity = '0';
            }
            
            this.element.style.transition = 'all 0.5s ease-in-out';
            this.element.style.width = '0px';
            this.element.style.height = '0px';

            setTimeout(() => {
                this.header.removeEventListener('mousedown', this.binds.headerMouseDown);
                this.drop.removeEventListener('click', this.binds.dropClick);
                this.close.removeEventListener('click', this.binds.closeClick);
                
                this.canvas.element.removeEventListener('mousemove', this.binds.headerMouseMove);
                this.header.removeEventListener('mouseup', this.binds.headerMouseUp);

                this.element.removeEventListener('mouseover', this.binds.mouseOver);
                this.element.removeEventListener('mouseout', this.binds.mouseOut);

                this.canvas.removeWidget(this);
            }, 500);
        }, 500);
    }

    dropClick(e) {
        this.focus();
        this.flags.expanded = !this.flags.expanded;
        this.element.style.height = this.flags.expanded ? this.positionInfo.resizing.currentSize.height + 'px' : '20px';
        this.drag.style.visibility = this.flags.expanded ? 'visible' : 'hidden';
        this.drop.style.transform = this.flags.expanded ? 'rotate(180deg)' : 'rotate(0deg)';
        for (const key in this.onExpand) {
            this.onExpand[key]();
        }

        if (this.flags.expanded) {
            this.drag.addEventListener('mousedown', this.binds.dragMouseDown);
        }
    }

    headerMouseDown(e) {
        this.focus();
        this.element.style.position = 'absolute';
        this.positionInfo.oldZIndex = this.element.style.zIndex;
        this.element.style.zIndex = 1000;

        this.positionInfo.movement.start.x = e.clientX - this.positionInfo.movement.current.x;
        this.positionInfo.movement.start.y = this.startY + (e.clientY - this.positionInfo.movement.current.y);

        if (!this.flags.hasBindedEvents) {
            this.flags.hasBindedEvents = true;
            this.flags.moving = true;
            this.moveAt(e);

            this.canvas.element.addEventListener('mousemove', this.binds.headerMouseMove);
            this.header.addEventListener('mouseup', this.binds.headerMouseUp);
        }
    }

    moveAt(e) {
        const mouse = {x: e.clientX, y: e.clientY};
        const dx = mouse.x - this.positionInfo.movement.start.x;
        const dy = mouse.y - this.positionInfo.movement.start.y;
        
        const sx = Math.round(dx / this.canvas.grid_size) * this.canvas.grid_size;
        const sy = Math.round(dy / this.canvas.grid_size) * this.canvas.grid_size;

        this.positionInfo.movement.current.x = sx;
        this.positionInfo.movement.current.y = this.startY + sy;

        this.element.style.left = sx + 'px';
        this.element.style.top = this.startY + sy + 'px';
    }

    headerMouseMove(e) {
        this.moveAt(e);
    }

    headerMouseUp() {
        this.canvas.element.removeEventListener('mousemove', this.binds.headerMouseMove);
        this.header.removeEventListener('mouseup', this.binds.headerMouseUp);
        this.flags.hasBindedEvents = false;
        this.flags.moving = false;
        this.element.style.zIndex = this.positionInfo.oldZIndex;
    }

    dragMouseDown(e) {
        this.focus();
        this.flags.dragging = true;
        this.positionInfo.resizing.start.x = e.clientX - this.positionInfo.resizing.current.x;
        this.positionInfo.resizing.start.y = e.clientY - this.positionInfo.resizing.current.y;

        this.canvas.element.addEventListener('mousemove', this.binds.dragMouseMove);
        this.drag.addEventListener('mouseup', this.binds.dragMouseUp);
    }

    dragMouseMove(e) {
        this.dragAt(e);
    }

    dragAt(e) {
        const dx = e.clientX - this.positionInfo.resizing.start.x;
        const dy = e.clientY - this.positionInfo.resizing.start.y;
        const sx = Math.round(dx / this.canvas.grid_size) * this.canvas.grid_size;
        const sy = Math.round(dy / this.canvas.grid_size) * this.canvas.grid_size;

        this.positionInfo.resizing.current.x = sx;
        this.positionInfo.resizing.current.y = sy;

        this.element.style.width = (this.width + sx) + 'px';
        this.element.style.height = (this.height + (sy - 19)) + 'px';

        this.positionInfo.resizing.currentSize.width = this.width + sx;
        this.positionInfo.resizing.currentSize.height = this.height + (sy - 20);
    }

    dragMouseUp() {
        this.canvas.element.removeEventListener('mousemove', this.binds.dragMouseMove);
        this.drag.removeEventListener('mouseup', this.binds.dragMouseUp);
        this.flags.dragging = false;
    }

    setCanvas(canvas) {
        this.canvas = canvas;
        this.canvas.element.appendChild(this.element);
    }

    focus() {   
        this.canvas.widgetFocused(this);
    }
}