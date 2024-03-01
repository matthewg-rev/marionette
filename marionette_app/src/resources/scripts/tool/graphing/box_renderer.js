class BoxRenderer extends Renderer {
    constructor() {
        super();

        this.size = 16;
        this.linePadding = 5;
        this.borderSize = 1;
        this.boxPadding = {
            x: 10,
            y: 20
        }
        this.boxShadow = 1;

        this.shadowOffset = 4;
        this.shadowBlur = 0;

        this.boxColors = {
            border: "#9b9b9b",
            background: "#0f0f0f",
            shadow: "#080808"
        }
    }

    longestText(ctx, text) {
        let longest = null;
        text.forEach((t) => {
            let metrics = ctx.measureText(t.raw());
            if (!longest || metrics.width > longest.width) {
                longest = metrics;
            }
        });
        return longest;
    }

    nodeSize(node, ctx) {
        ctx.font = this.size + "px Consolas";
        let metrics = this.longestText(ctx, node.data);
        let textHeight = this.textHeight(node, ctx);

        let width = metrics.width + (this.boxPadding.x * 2);
        let height = ((textHeight + this.linePadding) * (node.data.length-1)) + (this.boxPadding.y * 2);
        return {width, height};
    }

    textHeight(node, ctx) {
        ctx.font = this.size + "px Consolas";
        let metrics = this.longestText(ctx, node.data);
        return metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;
    }

    renderContents(node, ctx, nodeX, nodeY) {
        let textHeight = this.textHeight(node, ctx);
        // draw each line's text respecting padding and line height
        node.data.forEach((l, i) => {
            if (l instanceof Line) {
                let nextX = nodeX + this.boxPadding.x;
                l.content.forEach((t, j) => {
                    ctx.fillStyle = t.color;
                    ctx.fillText(
                        t.content, 
                        nextX, 
                        nodeY + this.boxPadding.y
                            + (i * textHeight) 
                            + (this.linePadding * (i+1))
                    );
                    nextX += ctx.measureText(t.content).width;
                });
            }
        });
    }

    renderBox(widget, ctx, nodeX, nodeY, width, height) {
        ctx.shadowColor = this.boxColors.shadow;
        ctx.shadowOffsetX = (this.shadowOffset) * widget.camera.zoom;
        ctx.shadowOffsetY = (this.shadowOffset) * widget.camera.zoom;
        ctx.shadowBlur = this.shadowBlur;

        ctx.fillStyle = this.boxColors.border;
        ctx.fillRect(
            nodeX - this.borderSize, 
            nodeY - this.borderSize, 
            width + (this.borderSize*2), 
            height + (this.borderSize*2)
        );

        ctx.shadowColor = "transparent";

        ctx.fillStyle = this.boxColors.background;
        ctx.fillRect(nodeX, nodeY, width, height);
    }
    
    render(node, widget, ctx) {
        // calculate width and height based on node.data
        let {width, height} = this.nodeSize(node, ctx);

        let nodeX = window.innerWidth/2;
        let nodeY = window.innerHeight/2;

        this.renderBox(widget, ctx, nodeX, nodeY, width, height);

        this.renderContents(node, ctx, nodeX, nodeY);
    }
}