class BoxRenderer extends Renderer {
    constructor(node) {
        super();
        this.node = node;

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
    
    render(widget, ctx) {
        // calculate width and height based on node.data
        ctx.font = this.size + "px Consolas";
        let metrics = this.longestText(ctx, this.node.data);
        let textHeight = metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;

        let width = metrics.width + (this.boxPadding.x * 2);
        let height = ((textHeight + this.linePadding) * (this.node.data.length-1)) + (this.boxPadding.y * 2);

        let nodeX = window.innerWidth/2;
        let nodeY = window.innerHeight/2;

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

        // draw each line's text respecting padding and line height
        this.node.data.forEach((l, i) => {
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
}